pub fn fetch_times(url: &str, numero_linea: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0...")
        .build()?;

    let response = client.get(url).send()?.text()?;
    let doc = dom_query::Document::from(&response);

    let filas = doc.select(".arrival_times_results_row");

    let mut tiempos_filtrados = Vec::new();

    for fila in filas.iter() {
        let texto_fila = fila.text().to_string();

        if texto_fila.contains(&format!("Línea {}", numero_linea)) || texto_fila.contains(&format!("L{}", numero_linea)) {
            let tiempo_span = fila.select("span.right").first();

            let t = tiempo_span.text().trim().to_string();
            if !t.is_empty() {
                tiempos_filtrados.push(t);
            }
        }
    }

    if tiempos_filtrados.is_empty() {
        Ok("Sin buses".to_string())
    } else {
        Ok(tiempos_filtrados.iter().take(2).cloned().collect::<Vec<_>>().join(" | "))
    }
}
