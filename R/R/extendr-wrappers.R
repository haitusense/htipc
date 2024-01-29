# nolint start

#' @docType package
#' @usage NULL
#' @useDynLib htipcR, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
namedpipe <- function(...) .Call(wrap__namedpipe, eval(substitute(alist(...))))

env <- function() .Call(wrap__env)
header <- function(path) .Call(wrap__header, path)

# nolint end
