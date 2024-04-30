# smooth-operator

Procedural macro that transforms regular infix arithmetic expressions into
checked arithmetic expressions.

## Example

The following invocation of `checked!()`:

```rs
fn the_answer() -> Result<i32, Error> {
    let answer = checked!(410 / 10 + 1)?;
    Ok(answer)
}
```

Results in this output:

```rs
fn the_answer() -> Result<i32, Error> {
    let answer = (|| -> ::core::result::Result<_, crate::Error> {
        type Err = crate::Error;
        const ORIGINAL_EXPR: &'static str = "410 / 10 + 1";
        Ok(
            #[allow(clippy::needless_question_mark)]
            #[allow(unused_parens)]
            {
                410.checked_div(10)
                    .ok_or(Err {
                        expr: ORIGINAL_EXPR,
                        __op_ix: 5usize,
                        __op_len: 1usize,
                    })?
                    .checked_add(1)
                    .ok_or(Err {
                        expr: ORIGINAL_EXPR,
                        __op_ix: 10usize,
                        __op_len: 1usize,
                    })?
            },
        )
    })()?;
    Ok(answer)
}
```
