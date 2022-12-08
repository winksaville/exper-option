use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exper_option::{
    check_box_of_val, check_box_of_val_ret_val_box, check_box_of_val_ret_val_option_box,
    check_option_box_of_val, check_option_box_of_val_ret_val_option_box, check_option_ptr_to_val,
    check_option_val, check_ptr_to_val, check_val,
};

#[allow(unused)]
fn bench_crit_check_option_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_val_ret_val", |b| {
        b.iter(|| black_box(check_option_val(Some(21))));
    });
}

#[allow(unused)]
fn bench_crit_check_option_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_val_ret_neg1", |b| {
        b.iter(|| black_box(check_option_val(Some(242))));
    });
}

#[allow(unused)]
fn bench_crit_check_option_val_ret_neg2(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_val_ret_neg2", |b| {
        b.iter(|| black_box(check_option_val(None)));
    });
}

#[allow(unused)]
fn bench_crit_check_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_val_ret_val", |b| {
        b.iter(|| black_box(check_val(21)));
    });
}

#[allow(unused)]
fn bench_crit_check_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_val_ret_neg1", |b| {
        b.iter(|| black_box(check_val(242)));
    });
}

#[allow(unused)]
fn bench_crit_check_option_ptr_to_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_ptr_to_val_ret_val", |b| {
        b.iter(|| {
            let val = 21;
            black_box(check_option_ptr_to_val(Some(&val)));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_ptr_to_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_ptr_to_val_ret_neg1", |b| {
        b.iter(|| {
            let val = 242;
            black_box(check_option_ptr_to_val(Some(&val)));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_ptr_to_val_ret_neg2(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_ptr_to_val_ret_neg2", |b| {
        b.iter(|| {
            black_box(check_option_ptr_to_val(None));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_ptr_to_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_ptr_to_val_ret_val", |b| {
        b.iter(|| {
            let val = 21;
            black_box(check_ptr_to_val(&val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_ptr_to_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_ptr_to_val_ret_neg1", |b| {
        b.iter(|| {
            let val = 242;
            black_box(check_ptr_to_val(&val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_box_of_val_ret_val", |b| {
        b.iter(|| {
            let val = Box::new(21);
            black_box(check_option_box_of_val(Some(val)));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_box_of_val_ret_neg1", |b| {
        b.iter(|| {
            let val = Box::new(242);
            black_box(check_option_box_of_val(Some(val)));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_neg2(c: &mut Criterion) {
    c.bench_function("bench_crit_check_option_box_of_val_ret_neg2", |b| {
        b.iter(|| {
            black_box(check_option_box_of_val(None));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val", |b| {
        b.iter(|| {
            let val = Box::new(21);
            black_box(check_box_of_val(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_neg1", |b| {
        b.iter(|| {
            let val = Box::new(242);
            black_box(check_box_of_val(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_val_option_box_val(c: &mut Criterion) {
    c.bench_function(
        "bench_crit_check_option_box_of_val_ret_val_option_box_val",
        |b| {
            b.iter(|| {
                let val = Box::new(21);
                black_box(check_option_box_of_val_ret_val_option_box(Some(val)));
            });
        },
    );
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_val_option_box_neg1(c: &mut Criterion) {
    c.bench_function(
        "bench_crit_check_option_box_of_val_ret_val_option_box_neg1",
        |b| {
            b.iter(|| {
                let val = Box::new(242);
                black_box(check_option_box_of_val_ret_val_option_box(Some(val)));
            });
        },
    );
}

#[allow(unused)]
fn bench_crit_check_option_box_of_val_ret_val_option_box_neg2(c: &mut Criterion) {
    c.bench_function(
        "bench_crit_check_option_box_of_val_ret_val_option_box_neg2",
        |b| {
            b.iter(|| {
                black_box(check_option_box_of_val_ret_val_option_box(None));
            });
        },
    );
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val_option_box_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val_option_box_val", |b| {
        b.iter(|| {
            let val = Box::new(21);
            black_box(check_box_of_val_ret_val_option_box(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val_option_box_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val_option_box_neg1", |b| {
        b.iter(|| {
            let val = Box::new(242);
            black_box(check_box_of_val_ret_val_option_box(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val_option_box_neg2(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val_option_box_neg2", |b| {
        b.iter(|| {
            let val = Box::new(128);
            black_box(check_box_of_val_ret_val_option_box(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val_box_val(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val_box_val", |b| {
        b.iter(|| {
            let val = Box::new(21);
            black_box(check_box_of_val_ret_val_box(val));
        });
    });
}

#[allow(unused)]
fn bench_crit_check_box_of_val_ret_val_box_neg1(c: &mut Criterion) {
    c.bench_function("bench_crit_check_box_of_val_ret_val_box_neg1", |b| {
        b.iter(|| {
            let val = Box::new(242);
            black_box(check_box_of_val_ret_val_box(val));
        });
    });
}

criterion_group!(
    benches,
    bench_crit_check_option_val_ret_val,
    bench_crit_check_option_val_ret_neg1,
    bench_crit_check_option_val_ret_neg2,
    bench_crit_check_val_ret_val,
    bench_crit_check_val_ret_neg1,
    bench_crit_check_option_ptr_to_val_ret_val,
    bench_crit_check_option_ptr_to_val_ret_neg1,
    bench_crit_check_option_ptr_to_val_ret_neg2,
    bench_crit_check_ptr_to_val_ret_val,
    bench_crit_check_ptr_to_val_ret_neg1,
    bench_crit_check_option_box_of_val_ret_val,
    bench_crit_check_option_box_of_val_ret_neg1,
    bench_crit_check_option_box_of_val_ret_neg2,
    bench_crit_check_box_of_val_ret_val,
    bench_crit_check_box_of_val_ret_neg1,
    bench_crit_check_option_box_of_val_ret_val_option_box_val,
    bench_crit_check_option_box_of_val_ret_val_option_box_neg1,
    bench_crit_check_option_box_of_val_ret_val_option_box_neg2,
    bench_crit_check_box_of_val_ret_val_option_box_val,
    bench_crit_check_box_of_val_ret_val_option_box_neg1,
    bench_crit_check_box_of_val_ret_val_option_box_neg2,
    bench_crit_check_box_of_val_ret_val_box_val,
    bench_crit_check_box_of_val_ret_val_box_neg1,
);
criterion_main!(benches);
