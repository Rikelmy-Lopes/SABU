use crate::analytics::dataset_analytics::VoteAnalytics;
use crate::analytics::vote_group::VoteGroup;
use crate::model::bulletin_record::BulletinRecord;

fn record(office: &str, party: &str, votable: &str, votes: i64) -> BulletinRecord {
    BulletinRecord {
        office_name: office.to_owned(),
        party_acronym: party.to_owned(),
        votable_name: votable.to_owned(),
        votes: Some(votes),
        ..BulletinRecord::default()
    }
}

#[test]
fn aggregates_and_sorts_by_votes() {
    let records = vec![
        record("Presidente", "PT", "Lula", 10),
        record("Presidente", "PT", "Lula", 5),
        record("Presidente", "PL", "Bolsonaro", 20),
        record("Governador", "PSDB", "João", 7),
    ];

    let analytics = VoteAnalytics::from_records(&records);

    assert_eq!(
        analytics.by_office,
        vec![
            VoteGroup {
                key: "Presidente".to_owned(),
                votes: 35
            },
            VoteGroup {
                key: "Governador".to_owned(),
                votes: 7
            },
        ]
    );

    assert_eq!(
        analytics.by_party,
        vec![
            VoteGroup {
                key: "PL".to_owned(),
                votes: 20
            },
            VoteGroup {
                key: "PT".to_owned(),
                votes: 15
            },
            VoteGroup {
                key: "PSDB".to_owned(),
                votes: 7
            },
        ]
    );
}

#[test]
fn ignores_empty_keys_and_missing_votes() {
    let mut without_votes = record("", "PT", "Lula", 10);
    without_votes.votes = None;

    let records = vec![without_votes, record("Presidente", "", "", 3)];

    let analytics = VoteAnalytics::from_records(&records);

    assert_eq!(
        analytics.by_office,
        vec![VoteGroup {
            key: "Presidente".to_owned(),
            votes: 3
        }]
    );
    assert!(analytics.by_party.is_empty());
    assert!(analytics.by_votable.is_empty());
}

#[test]
fn keeps_accented_keys() {
    let records = vec![record("Deputado Estadual", "PSOL", "Conceição", 12)];

    let analytics = VoteAnalytics::from_records(&records);

    assert_eq!(analytics.by_votable[0].key, "Conceição");
    assert_eq!(analytics.by_office[0].key, "Deputado Estadual");
}
