
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
    <title>{}</title>
</head>
<body>
    {} 
</body>
</html>
    ",
        back_route,
        title,
        body
    )
    
}
