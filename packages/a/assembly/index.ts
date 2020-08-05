// @ts-ignore: decorator
@external("b", "answerToLife")
declare function answerToLife(): i32;


if (answerToLife() != 42) {
  unreachable();
}

export function run(): i32 {
  return answerToLife();
}