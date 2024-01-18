# nolint start

#' @docType package
#' @usage NULL
#' @useDynLib htipcR, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
namedpipe <- function(path, value, ...) .Call(wrap__namedpipe, path, value, eval(substitute(alist(...))))

# nolint end
