use core::{panic};
use std::{os::windows::raw::SOCKET, vec};
use rand::{random, Rng};
const UNIT_NUM:i64=200;
const TIMES:i64=100;
const W:i32=60;
pub struct Puzzle{
    tasks:Vec<i32>,
    vms:Vec<i32>,
    expect:f32,
}
#[derive(Clone)]
pub struct Solution{
    pub assign:Vec<Vec<i32>>,
    task_weight:Vec<i32>,
    velocity:Vec<(i32,i32)>,
    max_response_time:f32,
}

impl Solution{
    pub fn new(vms:&Vec<i32>,tasks:&Vec<i32>)->Self{
        let mut a=vec![vec![-1;tasks.len()+1];vms.len()];
        let v=vec![(0,0);tasks.len()];
        for i in 0..vms.len(){
            a[i][0]=vms[i];
        }
        //TODO:应当改成随机分配
        let mut rng = rand::thread_rng();
        for i in 0..tasks.len(){
            loop{
                let rand_i=rng.gen_range(0..vms.len());
                let rand_j=rng.gen_range(1..tasks.len()+1);
                if a[rand_i][rand_j]!=-1{
                    continue;
                }else{
                    a[rand_i][rand_j]=i as i32;
                    break;
                }
            }
        }
        let mut tw=vec![0;tasks.len()];
        for i in 0..tasks.len(){
            tw[i]=tasks[i];
        }
        let mut output= Self{
            assign:a,
            task_weight:tw,
            velocity:v,
            max_response_time:0.0,
        };
        output.update_max_response_time();
        return output;
    }
    //TODO:有问题
    pub fn update_velocity(&mut self,best_solution:&Solution){
        //根据best更新速度
        for i in 0..self.velocity.len(){
            let mut rng = rand::thread_rng();
            let r: i32 = rng.gen_range(1..3);
            let (best_i,best_j)=find_index(i as i32, best_solution);
            let (my_i,my_j)=find_index(i as i32, &self);
            //添加惯性权重、学习因子
            let mut new_i_speed =self.velocity[i].0+ ((best_i as i32)-(my_i as i32))/r;
            let mut new_j_speed =self.velocity[i].0+((best_j as i32)-(my_j as i32))/r; 
            self.velocity[i]=adjust_speed((
                new_i_speed,new_j_speed), self.assign.len() as i32, self.assign[0].len() as i32
            );
        }
    }
    pub fn update_assign(&mut self){
        //TODO:没测试
        for task_num in 0..self.velocity.len(){
            let (i,j)=find_index(task_num as i32, &self);
            let mut new_i=i as i32+self.velocity[task_num].0;
            let mut new_j=j as i32+self.velocity[task_num].1;
            if new_i<0{
                new_i=0;
                self.velocity[task_num].0*=-1
            }
            if new_i>=self.assign.len() as i32{
                new_i=(self.assign.len()-1 )as i32;
                self.velocity[task_num].0*=-1
            }
            if new_j<1{
                new_j=1;
                self.velocity[task_num].1*=-1
            }
            if new_j>=self.assign[0].len() as i32{
                new_j=(self.assign[0].len()-1) as i32;
                self.velocity[task_num].1*=-1
            }
            if self.assign[new_i as usize][new_j as usize]==-1{
                self.assign[new_i as usize][new_j as usize]=self.assign[i][j];
            }else{
                let result=find_nearest_neg_one(&self.assign , i, j);
                match result{
                 Some(v)=>{
                    self.assign[v.0][v.1]=self.assign[i][j];
                    },
                 None=>{
                    panic!("move task failed");
                    }   
                }
            }
            self.assign[i][j]=-1;
        }
    }
    pub fn update_max_response_time(&mut self){
        self.max_response_time=0.0;
        for task_num in 1..self.velocity.len(){
            let (this_i,this_j)=find_index(task_num as i32, &self);
            let mut this_time:f32=0.0;
            for k in 1..this_j+1{
                if self.assign[this_i][k]==-1{
                    continue;
                }
                this_time+= self.task_weight[self.assign[this_i][k] as usize] as f32;
            }
            this_time/=self.assign[this_i][0] as f32;
            if this_time>self.max_response_time{
                self.max_response_time=this_time;
            }
        }
    }
    pub fn print(&self){
        for i in 0..self.assign.len(){
            for j in 0..self.assign[i].len(){
                if self.assign[i][j]==-1{
                    print!("_  ")
                }else{
                    print!{"{}  ",self.assign[i][j]};
                }
            }
            println!("");
        }
        println!("task weight:{:?}",self.task_weight);
        println!("velocity:{:?}",self.velocity);
        println!("max response time:{0}",self.max_response_time);
    }
    pub fn get_response_time(&self)->f32{
        return self.max_response_time;
    }

    pub fn print_assgin(&self){
        for i in 0..self.assign.len(){
            for j in 0..self.assign[i].len(){
                if self.assign[i][j]==-1{
                    print!("_  ")
                }else{
                    print!{"{}  ",self.assign[i][j]};
                }
            }
            println!("");
        }
        println!("\n");
    }
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

//TODO:完成tsa函数
pub fn do_tsa(puzzle:Puzzle)->Option<Solution>{
    let mut tasks=vec![Solution::new(&puzzle.vms, &puzzle.tasks);UNIT_NUM as usize];
    let mut best=Solution::new(&puzzle.vms, &puzzle.tasks);
    for _ in 0..TIMES{
        //TODO:记得关
        // println!("{}",tasks[0].get_response_time());
        // tasks[0].print_assgin();
        for i in 0..tasks.len(){
            tasks[i].update_velocity(&best);
            tasks[i].update_assign();
            tasks[i].update_max_response_time();
            if tasks[i].get_response_time()<best.get_response_time(){
                println!("best:{}",best.get_response_time());
                best=tasks[i].clone();
            }
        }
    }
    return Some(best);
}

//TODO:完成验证函数
pub fn valid_solution(puzzle:Puzzle,solution:Solution)->bool{
    return true;
}

fn find_index(task_num:i32,solution:&Solution)->(usize,usize){
    for i in 0..solution.assign.len(){
        for j in 1..solution.assign[0].len(){
            if solution.assign[i][j]==task_num{
                return (i,j)
            }
        }
    }
    return (0,0);
}

fn find_nearest_neg_one(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    queue.push_back((i, j));
    visited[i][j] = true;

    while let Some((x, y)) = queue.pop_front() {
        for dir in &directions {
            let new_x = x as i32 + dir.0;
            let new_y = y as i32 + dir.1;

            if new_x >= 0 && new_x < matrix.len() as i32 && new_y > 0 && new_y < matrix[0].len() as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;

                    if matrix[new_x][new_y] == -1 {
                        return Some((new_x, new_y));
                    } else {
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
    }

    None
}

fn adjust_speed(input: (i32,i32),row:i32,col:i32) -> (i32,i32) {
    let mut val=input;
    if val.0>=col{
        val.0=1;
    }
    if val.1>=row{
        val.1=1;
    }
    return val
}

