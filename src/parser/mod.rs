

use super::match_entry::MatchEntry;
use regex::Regex;

#[allow(dead_code)]
pub fn parse(data: String) -> Vec<MatchEntry> {
    let records: Vec<&str> = data.lines().collect();
    let mut entries = Vec::<MatchEntry>::with_capacity(records.len());
    let re = Regex::new(r"[:space:]?MATCH[:space:]([a-zA-Z_0-9]+)[,]+([0-9]+)[,]+([0-9]+)[,]+(.*)[,]+([a-zA-Z_0-9]+)[,]+.*").unwrap();
    for rec in records {
        for cap in re.captures_iter(rec) {
             entries.push(MatchEntry::new()
                                      .name(cap.at(1).unwrap_or("").to_string())
                                      .pos((
                                            (cap.at(2).unwrap()).to_string().parse::<u32>().unwrap()
                                           ,(cap.at(3).unwrap()).to_string().parse::<u32>().unwrap()
                                          ))
                                      .file(cap.at(4).unwrap_or("").to_string())
                                      .kind(cap.at(5).unwrap_or("").to_string())
                         );
        }
    }
    entries
}
