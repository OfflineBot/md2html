
#[allow(unused)]
pub fn html_template(back_route: String, title: String, body: String) -> String {
    format!(
    "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link rel=\"stylesheet\" href=\"{}css/styles.css\">
    <script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>
    <script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>
    <title>{}</title>
</head>
<body>
    <h1>{}</h1>
    {} 
</body>
</html>
    ",
        back_route,
        title,
        title,
        body
    )
    
}
