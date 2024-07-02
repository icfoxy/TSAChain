use core::{panic, task};
use std::{os::windows::raw::SOCKET, vec};
use rand::{random, Rng};
pub struct Puzzle{
    tasks:Vec<i32>,
    vms:Vec<i32>,
    expect:f32,
}
#[derive(Clone)]
pub struct Solution{
    pub assign:Vec<Vec<(i32,i32)>>,
    task_weight:Vec<i32>,
    velocity:Vec<(i32,i32)>,
    max_response_time:f32,
}

impl Puzzle {
    pub fn new(tasks:Vec<i32>,vms:Vec<i32>,expect:f32)->Self{
        return Puzzle{
            tasks:tasks,
            vms:vms,
            expect:expect,
        }
    }
}

impl Solution {
    pub fn new(vms:Vec<i32>,tasks:Vec<i32>)->Self{
        let mut assign=vec![vec![(-1,-1);tasks.len()+1];vms.len()];
        let velocity=vec![(0,0);tasks.len()];
        for i in 0..vms.len(){
            assign[i][0]=(vms[i],0);
        }
        //随机初始化
        let mut rng = rand::thread_rng();
        for i in 0..tasks.len(){
            loop{
                let rand_i=rng.gen_range(0..vms.len());
                let rand_j=rng.gen_range(1..tasks.len()+1);
                if assign[rand_i][rand_j]!=(-1,-1){
                    continue;
                }else{
                    assign[rand_i][rand_j]=(i as i32,-1);
                    break;
                }
            }
        }
        let mut output=Self{
            assign:assign,
            task_weight:tasks,
            velocity:velocity,
            max_response_time:0.0,
        };
        output.update_max_response_time();
        return output;
    }

    pub fn update_max_response_time(&mut self){
        self.max_response_time=0.0;
        for task_num in 0..self.task_weight.len(){
            let (this_i,this_j,this_z)=find_index(task_num as i32, &self.assign);
            let mut this_time:f32=0.0;
            for count in 1..this_j{
                if self.assign[this_i][count].0!=-1{
                    this_time+=self.task_weight[self.assign[this_i][count].0 as usize] as f32;
                }
                if self.assign[this_i][count].1!=-1{
                    this_time+=self.task_weight[self.assign[this_i][count].1 as usize] as f32;
                }
            }
            if this_z==0{
                this_time+=self.task_weight[self.assign[this_i][this_j].0 as usize] as f32
            }else{
                if self.assign[this_i][this_j].0!=-1{
                    this_time+=self.task_weight[self.assign[this_i][this_j].0 as usize] as f32;
                }
                this_time+=self.task_weight[self.assign[this_i][this_j].1 as usize] as f32;
            }
            this_time/=self.assign[this_i][0].0 as f32;
            if self.max_response_time<this_time{
                self.max_response_time=this_time;
            }
        }
    }

    pub fn update_velocity(&mut self,best_assign:&Vec<Vec<(i32,i32)>>){
        for task_num in 0..self.task_weight.len(){
            let mut rng = rand::thread_rng();
            let r: i32 = rng.gen_range(1..5);
            let (best_i,best_j,best_z)=find_index(task_num as i32, best_assign);
            let (this_i,this_j,this_z)=find_index(task_num as i32, &self.assign);
            let mut new_i_speed=(self.velocity[task_num].0)+(best_i as i32-this_i as i32)/r;
            let mut new_j_speed=self.velocity[task_num].1+(best_j as i32-this_j as i32)/r;
            let rand_i: i32 = rng.gen_range(-((self.assign.len()/2) as i32)..((self.assign.len()/2) as i32));
            let rand_j: i32 = rng.gen_range(-((self.assign[0].len()/2) as i32)..((self.assign[0].len()/2) as i32));
            if new_i_speed==0&&new_j_speed==0{
                new_i_speed=rand_i;
                new_j_speed=rand_j;
            }
            self.velocity[task_num]=(new_i_speed,new_j_speed);
        }
    }

    pub fn update_assign(&mut self){
        for task_num in 0..self.task_weight.len(){
            let (this_i,this_j,this_z)=find_index(task_num as i32, &self.assign);
            let new_i=this_i as i32+self.velocity[task_num].0;
            let new_j=this_j as i32+self.velocity[task_num].1;
            let (new_i_index,new_j_index,is_changed)=adjust_range(
                new_i, new_j, self.assign.len() as i32, self.assign[0].len() as i32
            );
            let mut rng = rand::thread_rng();
            let rand_i: i32 = rng.gen_range(-((self.assign.len()/2) as i32)..((self.assign.len()/2) as i32));
            let rand_j: i32 = rng.gen_range(-((self.assign[0].len()/2) as i32)..((self.assign[0].len()/2) as i32));
            if is_changed==1{
                self.velocity[task_num].0*=-1;
                self.velocity[task_num].1=rand_j;
            }else if is_changed==-1{
                self.velocity[task_num].1*=-1;
                self.velocity[task_num].0=rand_i;
            }
            self.move_task(task_num as i32, this_i, this_j, this_z, new_i_index, new_j_index);
            self.gravity();
        }
    }

    fn move_task(&mut self,task_num:i32,this_i:usize,this_j:usize,this_z:i32,new_i_index:usize,new_j_index:usize){
        if self.assign[new_i_index][new_j_index].0==-1{
            self.assign[new_i_index][new_j_index].0=task_num as i32;
        }else if self.assign[new_i_index][new_j_index].1==-1{
            self.assign[new_i_index][new_j_index].1=task_num as i32;
        }else{
            return;
        }

        if this_z==0{
            self.assign[this_i][this_j].0=-1;
        }else{
            self.assign[this_i][this_j].1=-1;
        }
    }

    pub fn print(&self) {
        for i in 0..self.assign.len() {
            for j in 0..self.assign[i].len() {
                if self.assign[i][j] == (-1, -1) {
                    print!("{: <8}", "_____");
                } else {
                    let val0 = if self.assign[i][j].0 == -1 { 
                        "_".to_string() 
                    } else { 
                        self.assign[i][j].0.to_string() 
                    };
                    let val1 = if self.assign[i][j].1 == -1 { 
                        "_".to_string() 
                    } else { 
                        self.assign[i][j].1.to_string() 
                    };
                    let output = format!("({}, {})", val0, val1);
                    print!("{: <8}", output);
                }
            }
            println!("");
        }
        println!("task weight: {:?}", self.task_weight);
        println!("velocity: {:?}", self.velocity);
        println!("max response time: {}", self.max_response_time);
    }

    pub fn gravity(&mut self){
        for i in 0..self.assign.len(){
            for j in 1..self.assign[0].len(){
                if self.assign[i][j].0==-1&&self.assign[i][j].1!=-1{
                    self.assign[i][j].0=self.assign[i][j].1;
                    self.assign[i][j].1=-1;
                }
            }
        }
    }

    pub fn get_max_response_time(&self)->f32{
        return self.max_response_time;
    }

    pub fn print_assign(&self){
        for i in 0..self.assign.len() {
            for j in 0..self.assign[i].len() {
                if self.assign[i][j] == (-1, -1) {
                    print!("{: <10}", "_____");
                } else {
                    let val0 = if self.assign[i][j].0 == -1 { 
                        "_".to_string() 
                    } else { 
                        self.assign[i][j].0.to_string() 
                    };
                    let val1 = if self.assign[i][j].1 == -1 { 
                        "_".to_string() 
                    } else { 
                        self.assign[i][j].1.to_string() 
                    };
                    let output = format!("({}, {})", val0, val1);
                    print!("{: <10}", output);
                }
            }
            println!("");
        }
        println!();
    }

    pub fn print_velocity(&self){
        println!("velocity: {:?}", self.velocity);
    }
}

pub fn do_tsa(puzzle:&Puzzle,unit_num:usize,times:usize)->Option<Solution>{
    let mut tasks=vec![Solution::new(puzzle.vms.clone(), puzzle.tasks.clone());unit_num];
    let mut best=Solution::new(puzzle.vms.clone(), puzzle.tasks.clone());
    for i in 0..tasks.len(){
        tasks[i].update_velocity(&best.assign);
    }
    for _ in 0..times{
        //TODO:记得关
        // println!("{}",tasks[0].get_response_time());
        // tasks[0].print_assign();
        // tasks[0].print_velocity();

        for i in 0..tasks.len(){
            tasks[i].update_assign();
            tasks[i].update_max_response_time();
            if tasks[i].get_max_response_time()<best.get_max_response_time(){
                println!("best:{}",best.get_max_response_time());
                best=tasks[i].clone();
                tasks[i].update_velocity(&best.assign);
            }
        }
    }
    return Some(best);
}
//TODO:
fn find_index(task_num:i32,assign:&Vec<Vec<(i32,i32)>>)->(usize,usize,i32){
    for i in 0..assign.len(){
        for j in 1..assign[0].len(){
            if assign[i][j].0==task_num{
                return (i,j,0);
            }
            if assign[i][j].1==task_num{
                return (i,j,1);
            }
        }
    }
    println!("{} not found",task_num);
    return (0,0,0)
}

fn adjust_range(i:i32,j:i32,row:i32,col:i32)->(usize,usize,i32){
    let mut out_i=i as usize;
    let mut out_j=j as usize;
    let mut is_changed=0;
    if i<0{
        out_i=0;
    }else if i>=row{
        out_i=(row-1) as usize-1;
    }
    if j<1{
        out_j=1;
    }else if j>=col{
        out_j=(col-1) as usize-1;
    }
    if out_i!=i as usize{
        is_changed=1;
    }else if out_j!=j as usize{
        is_changed=-1;
    }
    return (out_i,out_j,is_changed);
}