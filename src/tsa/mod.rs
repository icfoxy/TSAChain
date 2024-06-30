use core::task;

const UNIT_NUM:i64=2000;
const TIMES:i64=1000;
pub struct Puzzle{
    tasks:Vec<i32>,
    vms:Vec<i32>,
    expect:f32,
}
pub struct Solution{
    assign:Vec<Vec<i32>>,
    velocity:Vec<i32>,
    max_response_time:f32,
}

impl Solution{
    pub fn new(vms:&Vec<i32>,tasks:&Vec<i32>)->Self{
        let mut a=vec![vec![0;tasks.len()+1];vms.len()];
        let v=vec![0;tasks.len()];
        for i in 0..vms.len(){
            a[i][0]=vms[i];
        }
        //TODO:应当改成随机分配
        for i in 1..tasks.len()+1{
            a[0][i]=tasks[i-1];
        }
        return Self{
            assign:a,
            velocity:v,
            max_response_time:std::f32::MAX,
        }
    }
    pub fn update_velocity(&mut self,best_solution:&Solution){
        //TODO:没写呢
    }
    pub fn update_assign(&mut self){
        //TODO:没写呢
    }
    pub fn update_max_response_time(&mut self){
        //TODO:没写呢
    }
    pub fn print(&self){
        for i in 0..self.assign.len(){
            for j in 0..self.assign[i].len(){
                print!{"{} ",self.assign[i][j]};
            }
            println!("");
        }
    }

}

//TODO:完成tsa函数
pub fn do_tsa(puzzle:Puzzle)->Option<Solution>{
    return None;
}

//TODO:完成验证函数
pub fn valid_solution(puzzle:Puzzle,solution:Solution)->bool{
    return true;
}