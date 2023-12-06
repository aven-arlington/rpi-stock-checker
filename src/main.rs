use chrono::prelude::Local;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use windows::{
    core::*,
    Data::Xml::Dom::{IXmlNode, XmlDocument, XmlText},
    Foundation::{Collections::IVector, Uri},
    Web::Syndication::{SyndicationClient, SyndicationItem},
    UI::Notifications::{ToastNotification, ToastNotificationManager, ToastTemplateType},
};

fn main() -> Result<()> {
    // Setup the RSS feed reader
    let mut prev_feeds: HashSet<String> = HashSet::new();
    let uri = Uri::CreateUri(h!("https://rpilocator.com/feed"))?;
    let client = SyndicationClient::new()?;

    loop {
        // Get available feeds
        client.SetRequestHeader(
            h!("User-Agent"),
            h!("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)"),
        )?;
        let feed = client.RetrieveFeedAsync(&uri)?.get()?;

        // Parse the feeds and remove duplicates
        let feed_items: IVector<SyndicationItem> = feed.Items()?;
        for feed in feed_items {
            let feed_string: String = feed.Title()?.Text()?.to_string();
            if !prev_feeds.contains(&feed_string) {
                // New feed found
                let now = Local::now();
                prev_feeds.insert(feed_string.clone());
                println!(
                    "{} - New feed found: {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    feed_string
                );

                // Modify this search string to math the product you are looking for
                let search_string = "(US): RPi 5 - 8GB RAM";

                if feed_string.contains(search_string) {
                    // New feed detects search string. Send toast notification
                    let notification = {
                        let toast_xml: XmlDocument = ToastNotificationManager::GetTemplateContent(
                            ToastTemplateType::ToastText01,
                        )?;
                        let text_node: IXmlNode =
                            toast_xml.GetElementsByTagName(h!("text"))?.Item(0)?;
                        let text: XmlText =
                            toast_xml.CreateTextNode(&HSTRING::from(feed_string))?;
                        text_node.AppendChild(&text)?;
                        ToastNotification::CreateToastNotification(&toast_xml)?
                    };
                    ToastNotificationManager::GetDefault()?
                        .CreateToastNotifierWithId(h!(
                            "Microsoft.AutoGenerated.{A49227EA-5AF0-D494-A3F1-0918A278ED71}"
                        ))?
                        .Show(&notification)?;

                    // Allow time for async toast creation to complete
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                // Existing feed, ignore
                continue;
            }
        }

        // Update the stdout to show program is not idle/hung
        println!("Sleeping for 5 minutes");
        // Only check every 5 minutes to avoid spamming the RSS feed
        thread::sleep(Duration::from_secs(300));
    }
}
