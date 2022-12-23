mod queue;
mod stack;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stack_works() {
        use stack::{Stack, VecStack};
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);
        println!("{:#?}", stack);
        let i: Vec<&isize> = stack.iter().filter(|x| *x % 2 == 0).collect();
        println!("{:?}", i);
        stack.pop();
        stack.pop();
        println!("{:#?}", stack);
    }

    #[test]
    fn queue_works() {
        use queue::{DQueue, Queue};
        let mut q = DQueue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);
        println!("{:#?}", q);
        let i: Vec<&isize> = q.iter().filter(|x| *x % 2 == 0).collect();
        println!("{:?}", i);
        q.dequeue();
        q.dequeue();
        println!("{:#?}", q);
    }
}
