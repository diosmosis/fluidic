use lazy_static::lazy_static;
use jira_openapi_client::models::IssueBean;

lazy_static! {
    pub static ref TEST_ISSUES: Vec<IssueBean> = vec![
        serde_json::from_str::<IssueBean>(r#"
        {
           "expand": "operations,versionedRepresentations,editmeta,changelog,renderedFields",
           "id": "51783",
           "self": "https://innocraft.atlassian.net/rest/agile/1.0/issue/51783",
           "key": "MWP-757",
           "fields": {
             "statuscategorychangedate": "2024-10-02T02:29:23.417-0500",
             "customfield_10070": null,
             "customfield_10071": null,
             "customfield_10072": null,
             "customfield_10194": null,
             "customfield_10073": null,
             "fixVersions": [],
             "statusCategory": {
               "self": "https://innocraft.atlassian.net/rest/api/2/statuscategory/2",
               "id": 2,
               "key": "new",
               "colorName": "blue-gray",
               "name": "To Do"
             },
             "resolution": null,
             "lastViewed": "2025-03-11T11:04:50.381-0500",
             "customfield_10180": null,
             "customfield_10060": null,
             "epic": null,
             "priority": {
               "self": "https://innocraft.atlassian.net/rest/api/2/priority/3",
               "iconUrl": "https://innocraft.atlassian.net/images/icons/priorities/medium.svg",
               "name": "Medium",
               "id": "3"
             },
             "customfield_10068": null,
             "customfield_10189": null,
             "customfield_10069": null,
             "labels": [],
             "aggregatetimeoriginalestimate": null,
             "timeestimate": null,
             "versions": [],
             "issuelinks": [],
             "assignee": null,
             "status": {
               "self": "https://innocraft.atlassian.net/rest/api/2/status/10150",
               "description": "",
               "iconUrl": "https://innocraft.atlassian.net/images/icons/statuses/generic.png",
               "name": "Scheduled (in a sprint)",
               "id": "10150",
               "statusCategory": {
                 "self": "https://innocraft.atlassian.net/rest/api/2/statuscategory/2",
                 "id": 2,
                 "key": "new",
                 "colorName": "blue-gray",
                 "name": "To Do"
               }
             },
             "components": [],
             "customfield_10050": null,
             "customfield_10051": null,
             "customfield_10174": null,
             "customfield_10297": null,
             "customfield_10056": null,
             "customfield_10298": null,
             "customfield_10178": null,
             "customfield_10299": null,
             "customfield_10057": null,
             "customfield_10058": null,
             "customfield_10179": null,
             "customfield_10059": null,
             "customfield_10049": null,
             "aggregatetimeestimate": null,
             "creator": {
               "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
               "accountId": "5a9c69445cd11b2a721d9d0d",
               "emailAddress": "dizzy@innocraft.com",
               "avatarUrls": {
                 "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
               },
               "displayName": "Dizzy Faefyre",
               "active": true,
               "timeZone": "America/Chicago",
               "accountType": "atlassian"
             },
             "subtasks": [],
             "customfield_10040": null,
             "customfield_10041": null,
             "customfield_10042": null,
             "reporter": {
               "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
               "accountId": "5a9c69445cd11b2a721d9d0d",
               "emailAddress": "dizzy@innocraft.com",
               "avatarUrls": {
                 "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                 "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
               },
               "displayName": "Dizzy Faefyre",
               "active": true,
               "timeZone": "America/Chicago",
               "accountType": "atlassian"
             },
             "customfield_10043": null,
             "customfield_10044": null,
             "aggregateprogress": {
               "progress": 0,
               "total": 0
             },
             "customfield_10045": null,
             "customfield_10046": null,
             "customfield_10047": null,
             "customfield_10048": null,
             "customfield_10038": null,
             "customfield_10039": null,
             "closedSprints": [
               {
                 "id": 200,
                 "self": "https://innocraft.atlassian.net/rest/agile/1.0/sprint/200",
                 "state": "closed",
                 "name": "Matomo for Wordpress 6",
                 "startDate": "2024-04-15T11:40:00.663Z",
                 "endDate": "2024-04-30T05:00:00.000Z",
                 "completeDate": "2024-05-19T14:38:07.900Z",
                 "createdDate": "2024-02-26T22:17:19.039Z",
                 "originBoardId": 84,
                 "goal": ""
               }
             ],
             "progress": {
               "progress": 0,
               "total": 0
             },
             "votes": {
               "self": "https://innocraft.atlassian.net/rest/api/2/issue/MWP-757/votes",
               "votes": 0,
               "hasVoted": false
             },
             "worklog": {
               "startAt": 0,
               "maxResults": 20,
               "total": 0,
               "worklogs": []
             },
             "issuetype": {
               "self": "https://innocraft.atlassian.net/rest/api/2/issuetype/10005",
               "id": "10005",
               "description": "A small, distinct piece of work.",
               "iconUrl": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/issuetype/avatar/10318?size=medium",
               "name": "Task",
               "subtask": false,
               "avatarId": 10318,
               "hierarchyLevel": 0
             },
             "timespent": null,
             "sprint": null,
             "project": {
               "self": "https://innocraft.atlassian.net/rest/api/2/project/10079",
               "id": "10079",
               "key": "MWP",
               "name": "Matomo for WordPress",
               "projectTypeKey": "software",
               "simplified": false,
               "avatarUrls": {
                 "48x48": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401",
                 "24x24": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=small",
                 "16x16": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=xsmall",
                 "32x32": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=medium"
               }
             },
             "customfield_10032": null,
             "customfield_10033": null,
             "customfield_10034": null,
             "aggregatetimespent": null,
             "customfield_10035": null,
             "customfield_10036": null,
             "customfield_10037": null,
             "customfield_10302": null,
             "customfield_10303": null,
             "customfield_10304": null,
             "customfield_10305": null,
             "customfield_10306": null,
             "resolutiondate": null,
             "workratio": -1,
             "watches": {
               "self": "https://innocraft.atlassian.net/rest/api/2/issue/MWP-757/watchers",
               "watchCount": 2,
               "isWatching": true
             },
             "created": "2024-04-24T18:06:54.230-0500",
             "customfield_10020": null,
             "customfield_10021": null,
             "customfield_10022": null,
             "customfield_10300": null,
             "customfield_10301": null,
             "customfield_10016": null,
             "customfield_10017": null,
             "customfield_10018": null,
             "customfield_10019": null,
             "updated": "2025-03-11T11:30:05.961-0500",
             "customfield_10092": null,
             "timeoriginalestimate": null,
             "description": "Noticing a drop in overall users there (not on [plugins.wordpress.org|http://plugins.wordpress.org]), and want to see if it’s overall or just in countries that do not prioritize privacy (like the US).",
             "customfield_10010": [
               {
                 "id": 200,
                 "name": "Matomo for Wordpress 6",
                 "state": "closed",
                 "boardId": 84,
                 "goal": "",
                 "startDate": "2024-04-15T11:40:00.663Z",
                 "endDate": "2024-04-30T05:00:00.000Z",
                 "completeDate": "2024-05-19T14:38:07.900Z"
               }
             ],
             "customfield_10011": "0|hzzzyf:",
             "customfield_10012": "2024-07-10T02:01:15.128-0500",
             "customfield_10013": null,
             "customfield_10014": null,
             "timetracking": {},
             "customfield_10015": [],
             "security": null,
             "customfield_10008": null,
             "customfield_10009": {
               "hasEpicLinkFieldDependency": false,
               "showField": false,
               "nonEditableReason": {
                 "reason": "EPIC_LINK_SHOULD_BE_USED",
                 "message": "To set an epic as the parent, use the epic link instead"
               }
             },
             "attachment": [],
             "flagged": false,
             "summary": "scrape builtwith.com to get better usage analytics per country",
             "customfield_10080": null,
             "customfield_10081": null,
             "customfield_10082": null,
             "customfield_10083": null,
             "customfield_10085": null,
             "customfield_10086": null,
             "customfield_10000": "{}",
             "customfield_10122": null,
             "customfield_10001": null,
             "customfield_10123": null,
             "customfield_10004": [],
             "environment": null,
             "duedate": null,
             "comment": {
               "comments": [
                 {
                   "self": "https://innocraft.atlassian.net/rest/api/2/issue/51783/comment/190150",
                   "id": "190150",
                   "author": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                     "accountId": "5a9c69445cd11b2a721d9d0d",
                     "emailAddress": "dizzy@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                     },
                     "displayName": "Dizzy Faefyre",
                     "active": true,
                     "timeZone": "America/Chicago",
                     "accountType": "atlassian"
                   },
                   "body": "[~accountid:712020:00df85f1-2dd4-42e8-8a88-c81b2c9c5443] [~accountid:5a98a1ee4af2372a88a00c78] [~accountid:5a989d770f36f12a6c56af3f] (there is a question for you at the bottom of this)\n\nI’ve created a web scraper at: [https://github.com/innocraft/mwp-feedback-scraper/|https://github.com/innocraft/mwp-feedback-scraper/|smart-link] that creates a dataset with MWP related product insights (and possibly some related to the core product).\n\nThis scraper creates a dataset that includes:\n\n* every [wordpress.org|http://wordpress.org] review for a plugin (including rating, text and comments)\n* every [wordpress.org|http://wordpress.org] support request for a plugin (including text and comments)\n* active installations for a plugin + the short description of a plugin\n* the builtwith stats (total and by country) of domains that use a plugin (for plugins that have these stats)\n\nThe dataset includes this information for Matomo for WordPress & competitor plugins.\n\nI will be using the data to try and understand what WP users want/hate/etc. from an analytics solution as well as track what users are using over time (so we’re not shipping MWP blind).\n\nMy question is, would the product team want access to this data at some point? And would they want some sort of viewer for it (the dataset will just be raw data so-to-speak, so it won’t be human readable by itself)?",
                   "updateAuthor": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                     "accountId": "5a9c69445cd11b2a721d9d0d",
                     "emailAddress": "dizzy@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                     },
                     "displayName": "Dizzy Faefyre",
                     "active": true,
                     "timeZone": "America/Chicago",
                     "accountType": "atlassian"
                   },
                   "created": "2024-07-10T01:53:58.190-0500",
                   "updated": "2024-07-10T01:59:53.243-0500",
                   "jsdPublic": true
                 },
                 {
                   "self": "https://innocraft.atlassian.net/rest/api/2/issue/51783/comment/190151",
                   "id": "190151",
                   "author": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a98a1ee4af2372a88a00c78",
                     "accountId": "5a98a1ee4af2372a88a00c78",
                     "emailAddress": "matt@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/48",
                       "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/24",
                       "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/16",
                       "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/32"
                     },
                     "displayName": "Matthieu Aubry",
                     "active": true,
                     "timeZone": "Pacific/Auckland",
                     "accountType": "atlassian"
                   },
                   "body": "[~accountid:5a9c69445cd11b2a721d9d0d] thank you for asking. \n\nAt a minimum I would suggest to create a simple wikipage to explain what you have done and why, and link to the github, so there is a trace of the project.\n\n\n\nand then maybe the raw data you have could also be put into a Excel sheet or something that is easy to look at if needed later?(Excel sheet that can be linked from the wiki and GitHub)\n\n  But if that's not easily done it is not a problem and we can see later if people would need the data and ignore it for now? ",
                   "updateAuthor": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a98a1ee4af2372a88a00c78",
                     "accountId": "5a98a1ee4af2372a88a00c78",
                     "emailAddress": "matt@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/48",
                       "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/24",
                       "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/16",
                       "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/5a98a1ee4af2372a88a00c78/fee1f648-f229-440d-b646-c16b201ede67/32"
                     },
                     "displayName": "Matthieu Aubry",
                     "active": true,
                     "timeZone": "Pacific/Auckland",
                     "accountType": "atlassian"
                   },
                   "created": "2024-07-10T02:01:15.128-0500",
                   "updated": "2024-07-10T02:01:15.128-0500",
                   "jsdPublic": true
                 },
                 {
                   "self": "https://innocraft.atlassian.net/rest/api/2/issue/51783/comment/190153",
                   "id": "190153",
                   "author": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                     "accountId": "5a9c69445cd11b2a721d9d0d",
                     "emailAddress": "dizzy@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                     },
                     "displayName": "Dizzy Faefyre",
                     "active": true,
                     "timeZone": "America/Chicago",
                     "accountType": "atlassian"
                   },
                   "body": "Yes, there will be a wiki when I’m done.\n\nI assume from your answer it would not be useful to the product team?",
                   "updateAuthor": {
                     "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                     "accountId": "5a9c69445cd11b2a721d9d0d",
                     "emailAddress": "dizzy@innocraft.com",
                     "avatarUrls": {
                       "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                       "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                     },
                     "displayName": "Dizzy Faefyre",
                     "active": true,
                     "timeZone": "America/Chicago",
                     "accountType": "atlassian"
                   },
                   "created": "2024-07-10T02:11:07.590-0500",
                   "updated": "2024-07-10T02:11:07.590-0500",
                   "jsdPublic": true
                 }
               ],
               "self": "https://innocraft.atlassian.net/rest/api/2/issue/51783/comment",
               "maxResults": 3,
               "total": 3,
               "startAt": 0
             }
           }
         },
        "#).unwrap(),

        serde_json::from_str::<IssueBean>(r#"
        {
          "expand": "operations,versionedRepresentations,editmeta,changelog,renderedFields",
          "id": "51817",
          "self": "https://innocraft.atlassian.net/rest/agile/1.0/issue/51817",
          "key": "MWP-764",
          "fields": {
            "statuscategorychangedate": "2024-05-07T13:20:03.540-0500",
            "customfield_10070": null,
            "customfield_10071": null,
            "customfield_10072": null,
            "customfield_10194": null,
            "customfield_10073": null,
            "fixVersions": [],
            "statusCategory": {
              "self": "https://innocraft.atlassian.net/rest/api/2/statuscategory/2",
              "id": 2,
              "key": "new",
              "colorName": "blue-gray",
              "name": "To Do"
            },
            "resolution": null,
            "lastViewed": null,
            "customfield_10180": null,
            "customfield_10060": null,
            "epic": null,
            "priority": {
              "self": "https://innocraft.atlassian.net/rest/api/2/priority/3",
              "iconUrl": "https://innocraft.atlassian.net/images/icons/priorities/medium.svg",
              "name": "Medium",
              "id": "3"
            },
            "customfield_10068": null,
            "customfield_10189": null,
            "customfield_10069": null,
            "labels": [],
            "aggregatetimeoriginalestimate": null,
            "timeestimate": null,
            "versions": [],
            "issuelinks": [],
            "assignee": null,
            "status": {
              "self": "https://innocraft.atlassian.net/rest/api/2/status/10150",
              "description": "",
              "iconUrl": "https://innocraft.atlassian.net/images/icons/statuses/generic.png",
              "name": "Scheduled (in a sprint)",
              "id": "10150",
              "statusCategory": {
                "self": "https://innocraft.atlassian.net/rest/api/2/statuscategory/2",
                "id": 2,
                "key": "new",
                "colorName": "blue-gray",
                "name": "To Do"
              }
            },
            "components": [],
            "customfield_10050": null,
            "customfield_10051": null,
            "customfield_10174": null,
            "customfield_10297": null,
            "customfield_10056": null,
            "customfield_10298": null,
            "customfield_10178": null,
            "customfield_10299": null,
            "customfield_10057": null,
            "customfield_10058": null,
            "customfield_10179": null,
            "customfield_10059": null,
            "customfield_10049": null,
            "aggregatetimeestimate": null,
            "creator": {
              "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
              "accountId": "5a9c69445cd11b2a721d9d0d",
              "emailAddress": "dizzy@innocraft.com",
              "avatarUrls": {
                "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
              },
              "displayName": "Dizzy Faefyre",
              "active": true,
              "timeZone": "America/Chicago",
              "accountType": "atlassian"
            },
            "subtasks": [],
            "customfield_10040": null,
            "customfield_10041": null,
            "customfield_10042": null,
            "reporter": {
              "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
              "accountId": "5a9c69445cd11b2a721d9d0d",
              "emailAddress": "dizzy@innocraft.com",
              "avatarUrls": {
                "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
              },
              "displayName": "Dizzy Faefyre",
              "active": true,
              "timeZone": "America/Chicago",
              "accountType": "atlassian"
            },
            "customfield_10043": null,
            "customfield_10044": null,
            "aggregateprogress": {
              "progress": 0,
              "total": 0
            },
            "customfield_10045": null,
            "customfield_10046": null,
            "customfield_10047": null,
            "customfield_10048": null,
            "customfield_10038": null,
            "customfield_10039": null,
            "closedSprints": [
              {
                "id": 200,
                "self": "https://innocraft.atlassian.net/rest/agile/1.0/sprint/200",
                "state": "closed",
                "name": "Matomo for Wordpress 6",
                "startDate": "2024-04-15T11:40:00.663Z",
                "endDate": "2024-04-30T05:00:00.000Z",
                "completeDate": "2024-05-19T14:38:07.900Z",
                "createdDate": "2024-02-26T22:17:19.039Z",
                "originBoardId": 84,
                "goal": ""
              }
            ],
            "progress": {
              "progress": 0,
              "total": 0
            },
            "votes": {
              "self": "https://innocraft.atlassian.net/rest/api/2/issue/MWP-764/votes",
              "votes": 0,
              "hasVoted": false
            },
            "worklog": {
              "startAt": 0,
              "maxResults": 20,
              "total": 0,
              "worklogs": []
            },
            "issuetype": {
              "self": "https://innocraft.atlassian.net/rest/api/2/issuetype/10005",
              "id": "10005",
              "description": "A small, distinct piece of work.",
              "iconUrl": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/issuetype/avatar/10318?size=medium",
              "name": "Task",
              "subtask": false,
              "avatarId": 10318,
              "hierarchyLevel": 0
            },
            "timespent": null,
            "sprint": null,
            "project": {
              "self": "https://innocraft.atlassian.net/rest/api/2/project/10079",
              "id": "10079",
              "key": "MWP",
              "name": "Matomo for WordPress",
              "projectTypeKey": "software",
              "simplified": false,
              "avatarUrls": {
                "48x48": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401",
                "24x24": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=small",
                "16x16": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=xsmall",
                "32x32": "https://innocraft.atlassian.net/rest/api/2/universal_avatar/view/type/project/avatar/10401?size=medium"
              }
            },
            "customfield_10032": null,
            "customfield_10033": null,
            "customfield_10034": null,
            "aggregatetimespent": null,
            "customfield_10035": null,
            "customfield_10036": null,
            "customfield_10037": null,
            "customfield_10302": null,
            "customfield_10303": null,
            "customfield_10304": null,
            "customfield_10305": null,
            "customfield_10306": null,
            "resolutiondate": null,
            "workratio": -1,
            "watches": {
              "self": "https://innocraft.atlassian.net/rest/api/2/issue/MWP-764/watchers",
              "watchCount": 1,
              "isWatching": true
            },
            "created": "2024-04-28T08:58:55.579-0500",
            "customfield_10020": null,
            "customfield_10021": null,
            "customfield_10022": null,
            "customfield_10300": null,
            "customfield_10301": null,
            "customfield_10016": null,
            "customfield_10017": null,
            "customfield_10018": null,
            "customfield_10019": null,
            "updated": "2025-03-11T11:05:10.616-0500",
            "customfield_10092": null,
            "timeoriginalestimate": null,
            "description": "There’s some text in the WP plugin guidelines about not including iframes in WP admin back office pages. It seems to be targeting loading external domains with iframes, but this is not clear in the wording. So we should see if we can embed the Matomo UI directly into the WP back office.",
            "customfield_10010": [
              {
                "id": 200,
                "name": "Matomo for Wordpress 6",
                "state": "closed",
                "boardId": 84,
                "goal": "",
                "startDate": "2024-04-15T11:40:00.663Z",
                "endDate": "2024-04-30T05:00:00.000Z",
                "completeDate": "2024-05-19T14:38:07.900Z"
              }
            ],
            "customfield_10011": "0|hzzzyv:",
            "customfield_10012": null,
            "customfield_10013": null,
            "customfield_10014": null,
            "timetracking": {},
            "customfield_10015": [],
            "security": null,
            "customfield_10008": null,
            "customfield_10009": {
              "hasEpicLinkFieldDependency": false,
              "showField": false,
              "nonEditableReason": {
                "reason": "EPIC_LINK_SHOULD_BE_USED",
                "message": "To set an epic as the parent, use the epic link instead"
              }
            },
            "attachment": [],
            "flagged": false,
            "summary": "try removing iframe use in measurable settings page",
            "customfield_10080": null,
            "customfield_10081": null,
            "customfield_10082": null,
            "customfield_10083": null,
            "customfield_10085": null,
            "customfield_10086": null,
            "customfield_10000": "{}",
            "customfield_10122": null,
            "customfield_10001": null,
            "customfield_10123": null,
            "customfield_10004": [],
            "environment": null,
            "duedate": null,
            "comment": {
              "comments": [
                {
                  "self": "https://innocraft.atlassian.net/rest/api/2/issue/51817/comment/184662",
                  "id": "184662",
                  "author": {
                    "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                    "accountId": "5a9c69445cd11b2a721d9d0d",
                    "emailAddress": "dizzy@innocraft.com",
                    "avatarUrls": {
                      "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                    },
                    "displayName": "Dizzy Faefyre",
                    "active": true,
                    "timeZone": "America/Chicago",
                    "accountType": "atlassian"
                  },
                  "body": "According to the WP plugins team, our use case is fine. I’ll still be looking into the possibilities here though, as it would provide a better experience overall.",
                  "updateAuthor": {
                    "self": "https://innocraft.atlassian.net/rest/api/2/user?accountId=5a9c69445cd11b2a721d9d0d",
                    "accountId": "5a9c69445cd11b2a721d9d0d",
                    "emailAddress": "dizzy@innocraft.com",
                    "avatarUrls": {
                      "48x48": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "24x24": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "16x16": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png",
                      "32x32": "https://secure.gravatar.com/avatar/a357f5f915c003211d18ab7c1c00b3a3?d=https%3A%2F%2Favatar-management--avatars.us-west-2.prod.public.atl-paas.net%2Finitials%2FDF-0.png"
                    },
                    "displayName": "Dizzy Faefyre",
                    "active": true,
                    "timeZone": "America/Chicago",
                    "accountType": "atlassian"
                  },
                  "created": "2024-04-29T10:37:58.360-0500",
                  "updated": "2024-04-29T10:37:58.360-0500",
                  "jsdPublic": true
                }
              ],
              "self": "https://innocraft.atlassian.net/rest/api/2/issue/51817/comment",
              "maxResults": 1,
              "total": 1,
              "startAt": 0
            }
          }
        }"#).unwrap(),
    ];
}
