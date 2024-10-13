struct Task{
    start_time: u32,
    end_time: u32,
    cookies: u32,
}

/*
? Maybe sort by end_time.
Get task with closest end time.
Try all tasks with start_time less than that one's end_time.
Recurse.
*/
#[allow(dead_code)]
fn time_manager(tasks: &mut Vec<Task>) -> u32 {

}

fn get_closest_end_time(tasks: &Vec<Task>, cur_time: u32) -> &Task {
    let best: Option<&Task> = None;

    for task in tasks {
        if task.start_time >= cur_time && task.end_time < best.end_time {
            return &task;
        }
    }
    best
}

fn get_closest_start_time(tasks: &Vec<Task>, cur_time: u32) -> &Task {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
