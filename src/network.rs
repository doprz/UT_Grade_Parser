use std::fs::{create_dir_all, File};
use std::io::Write;

use bytes::Bytes;

async fn get_session_id() -> Result<String, reqwest::Error> {
    let url: &str = "https://iq-analytics.austin.utexas.edu/views/Gradedistributiondashboard/Externaldashboard-Crosstab?%3Aembed=y&%3AisGuestRedirectFromVizportal=n";
    let response: reqwest::Response = reqwest::get(url).await?;
    let body: String = response.text().await?;

    let document: scraper::Html = scraper::Html::parse_document(&body);
    let selector: scraper::Selector = scraper::Selector::parse("#tsConfigContainer").unwrap();
    let mut result: scraper::html::Select<'_, '_> = document.select(&selector);

    // JSON.parse(document.getElementById('tsConfigContainer').value).sessionid;
    let json_str: String = result.next().unwrap().inner_html();
    let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    let session_id: &str = json_value["sessionid"].as_str().unwrap();

    Ok(session_id.to_string())
}

async fn bootstrap(session_id: &str) -> Result<(), reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/bootstrapSession/sessions/{}", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("sheet_id=External%20dashboard-Crosstab")
        .send()
        .await?;

    Ok(())
}

async fn get_sheet_doc_id(session_id: &str) -> Result<String, reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/sessions/{}/commands/tabsrv/export-crosstab-server-dialog", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    let raw_multipart: &str = r#"{"External dashboard-Crosstab":"/thumb/views/Gradedistributiondashboard/Externaldashboard-Crosstab","External dashboard-bar graph":"/thumb/views/Gradedistributiondashboard/Externaldashboard-bargraph"}"#;
    let multipart: reqwest::multipart::Form =
        reqwest::multipart::Form::new().text("thumbnailUris", raw_multipart);
    let response: reqwest::Response = client.post(&url).multipart(multipart).send().await?;

    let json_response: serde_json::Value = response.text().await.unwrap().parse().unwrap();
    // println!("{:#?}", json_response);

    let sheet_doc_id: &serde_json::Value = &json_response["vqlCmdResponse"]["layoutStatus"]
        ["applicationPresModel"]["presentationLayerNotification"][0]["presModelHolder"]
        ["genExportCrosstabOptionsDialogPresModel"]["thumbnailSheetPickerItems"][0]["sheetdocId"];

    Ok(sheet_doc_id.as_str().unwrap().to_string())
}

async fn categorial_filter_all(
    session_id: &str,
    global_field_name: &str,
) -> Result<(), reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/sessions/{}/commands/tabdoc/categorical-filter", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    // Select all semesters and select all courses
    let multipart: reqwest::multipart::Form = reqwest::multipart::Form::new()
        .text("visualIdPresModel", r#"{"worksheet":"Grade distribution - external","dashboard":"External dashboard-Crosstab"}"#)
        .text("membershipTarget", "filter")
        .text("globalFieldName", format!("[sqlproxy.1nikk2j199ysrw13cof5d1qn00ff].[none:{}:nk]", global_field_name.to_string()))
        .text("filterValues", "[]")
        .text("filterUpdateType", "filter-all");

    client.post(&url).multipart(multipart).send().await?;

    Ok(())
}

async fn categorical_filter_indices(
    session_id: &str,
    global_field_name: &str,
    index: usize,
) -> Result<(), reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/sessions/{}/commands/tabdoc/categorical-filter-by-index", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    // Select all semesters
    let multipart: reqwest::multipart::Form = reqwest::multipart::Form::new()
        .text("visualIdPresModel", r#"{"worksheet":"Grade distribution - external","dashboard":"External dashboard-Crosstab"}"#)
        .text("membershipTarget", "filter")
        .text("globalFieldName", format!("[sqlproxy.1nikk2j199ysrw13cof5d1qn00ff].[none:{}:nk]", global_field_name.to_string()))
        .text("filterIndices", format!("[{}]", index))
        .text("filterUpdateType", "filter-replace");

    client.post(&url).multipart(multipart).send().await?;

    Ok(())
}

async fn set_expanded_values(session_id: &str) -> Result<(), reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/sessions/{}/commands/tabdoc/set-parameter-value", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    // Select all semesters
    let multipart: reqwest::multipart::Form = reqwest::multipart::Form::new()
        .text("globalFieldName", "[Parameters].[Parameter 1]")
        .text("valueString", "Expanded")
        .text("useUsLocale", "false");

    client.post(&url).multipart(multipart).send().await?;

    Ok(())
}

async fn get_export_result_key(
    session_id: &str,
    sheet_doc_id: &str,
) -> Result<String, reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/sessions/{}/commands/tabsrv/export-crosstab-to-csvserver", session_id);
    let client: reqwest::Client = reqwest::Client::new();

    let multipart: reqwest::multipart::Form = reqwest::multipart::Form::new()
        .text("sheetdocId", sheet_doc_id.to_string())
        .text("useTabs", "false")
        .text("sendNotifications", "false");
    let response: reqwest::Response = client.post(&url).multipart(multipart).send().await?;

    let json_response: serde_json::Value = response.text().await.unwrap().parse().unwrap();

    let result_key: &serde_json::Value = &json_response["vqlCmdResponse"]["cmdResultList"][0]
        ["commandReturn"]["exportResult"]["resultKey"];

    Ok(result_key.as_str().unwrap().to_string())
}

async fn download_exported_csv(
    session_id: &str,
    result_key: &str,
) -> Result<Bytes, reqwest::Error> {
    let url: String = format!("https://iq-analytics.austin.utexas.edu/vizql/w/Gradedistributiondashboard/v/Externaldashboard-Crosstab/tempfile/sessions/{}/?key={}", session_id, result_key);
    let response: reqwest::Response = reqwest::get(&url).await?;
    let body: Bytes = response.bytes().await?;

    Ok(body)
}

async fn export_csv(session_id: &str) -> Result<Bytes, reqwest::Error> {
    let sheet_doc_id: String = get_sheet_doc_id(session_id).await?;
    // println!("Sheet Doc ID: {}", sheet_doc_id);
    let result_key: String = get_export_result_key(session_id, &sheet_doc_id).await?;
    // println!("Result Key: {}", result_key);
    let csv = download_exported_csv(session_id, &result_key).await?;

    Ok(csv)
}

pub async fn fetch_and_download_grade_distributions() -> Result<(), Box<dyn std::error::Error>> {
    let session_id: String = get_session_id().await?;
    println!("Session ID: {}", session_id);

    println!("[1/4] Bootstrapping session");
    bootstrap(&session_id).await?;

    // filter sheet
    println!("[2/4] Categorial filter all. Select all semesters and select all courses");
    categorial_filter_all(&session_id, "Calculation_3161245480939225089").await?;
    categorial_filter_all(&session_id, "COURSE_PREFIX").await?;

    println!("[3/4] Set expanded values");
    set_expanded_values(&session_id).await?;

    create_dir_all("out")?;

    // 2010-2011 to 2022-2023
    let years: usize = 13;

    println!("[4/4] Exporting CSVs");
    let pb = indicatif::ProgressBar::new(years as u64);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    for i in 0..years {
        pb.set_message(format!("Exporting CSV for {}-{}", i + 2010, i + 2011));
        categorical_filter_indices(&session_id, "ACADEMIC_YEAR_SPAN", i).await?;
        let csv = export_csv(&session_id).await?;

        let file_name = format!("out/grade_distributions_{}-{}.csv", i + 2010, i + 2011);
        let mut file = File::create(file_name)?;
        file.write_all(&csv)?;

        pb.inc(1);
    }
    pb.finish_with_message("Exported all CSVs");

    Ok(())
}
