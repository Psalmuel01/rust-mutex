// skip-filecheck
//@ unit-test: EarlyOtherwiseBranch
//@ compile-flags: -Zmir-enable-passes=+UnreachableEnumBranching

enum Option2<T> {
    Some(T),
    None,
    Other,
}

// We can't optimize it because y may be an invalid value.
// EMIT_MIR early_otherwise_branch.opt1.EarlyOtherwiseBranch.diff
fn opt1(x: Option<u32>, y: Option<u32>) -> u32 {
    match (x, y) {
        (Some(a), Some(b)) => 0,
        _ => 1,
    }
}

// FIXME: `switchInt` will have three targets after `UnreachableEnumBranching`,
// otherwise is unreachable. We can consume the UB fact to transform back to if else pattern.
// EMIT_MIR early_otherwise_branch.opt2.EarlyOtherwiseBranch.diff
fn opt2(x: Option<u32>, y: Option<u32>) -> u32 {
    match (x, y) {
        (Some(a), Some(b)) => 0,
        (None, None) => 2,
        _ => 1,
    }
}

// optimize despite different types
// EMIT_MIR early_otherwise_branch.opt3.EarlyOtherwiseBranch.diff
fn opt3(x: Option2<u32>, y: Option2<bool>) -> u32 {
    match (x, y) {
        (Option2::Some(a), Option2::Some(b)) => 0,
        (Option2::None, Option2::None) => 2,
        (Option2::Other, Option2::Other) => 3,
        _ => 1,
    }
}

// EMIT_MIR early_otherwise_branch.opt4.EarlyOtherwiseBranch.diff
fn opt4(x: Option2<u32>, y: Option2<u32>) -> u32 {
    match (x, y) {
        (Option2::Some(a), Option2::Some(b)) => 0,
        (Option2::None, Option2::None) => 2,
        (Option2::Other, Option2::Other) => 3,
        _ => 1,
    }
}

fn main() {
    opt1(None, Some(0));
    opt2(None, Some(0));
    opt3(Option2::None, Option2::Some(false));
    opt4(Option2::None, Option2::Some(0));
}
