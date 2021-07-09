#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A single AST expression tree
 *
 * This is parsed from text by [`parse()`](crate::parse::parse()), and evaluated
 * into an `i64` by `eval()`.
 */
typedef struct Expr Expr;

/**
 * This will return null if unsuccessful
 */
struct Expr *c_parse(const char *maybe_cstr);

intptr_t c_eval(const struct Expr *expr, int64_t *output);

intptr_t parse_and_eval(const char *maybe_cstr, int64_t *output);

void release_expr(struct Expr *box_expr);
