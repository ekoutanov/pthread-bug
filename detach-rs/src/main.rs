#![feature(bench_black_box)]

use hint::black_box;
use std::{hint, thread};

fn main() {
    for _ in 0..1000_000 {
        let h_1 = thread::spawn(|| {});
        let h_2 = thread::spawn(|| {});
        let h_3 = thread::spawn(|| {});
        let h_4 = thread::spawn(|| {});

        black_box(&h_1);
        black_box(&h_2);
        black_box(&h_3);
        black_box(&h_4);

        drop(h_1);
        drop(h_2);
        drop(h_3);
        drop(h_4);
    }
}


// #include <pthread.h>
// #include <assert.h>
//
// void *start(void *arg)
// {
// return 0;
// }
//
// int main()
// {
// int h;
// for (h = 0; h < 100000; h++) {
// int i;
// pthread_attr_t attr;
// assert(pthread_attr_init(&attr) == 0);
// assert(pthread_attr_setstacksize(&attr, 50*1024*1024) == 0);
//
// pthread_t handle1;
// pthread_t handle2;
// pthread_t handle3;
// pthread_t handle4;
// assert(pthread_create(&handle1, &attr, start, 0) == 0);
// assert(pthread_create(&handle2, &attr, start, 0) == 0);
// assert(pthread_create(&handle3, &attr, start, 0) == 0);
// assert(pthread_create(&handle4, &attr, start, 0) == 0);
// assert(pthread_detach(handle1) == 0);
// assert(pthread_detach(handle2) == 0);
// assert(pthread_detach(handle3) == 0);
// assert(pthread_detach(handle4) == 0);
// }
// return 0;
// }