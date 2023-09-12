# KeywordTracker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tracker_id** | Option<**String**> | The unique identifier of the tracker. | [optional]
**tracker_name** | Option<**String**> | The name of the tracker. | [optional]
**workspace_id** | Option<**String**> | The id of the workspace the tracker is in. | [optional]
**language_keywords** | Option<[**Vec<crate::models::TrackerLanguageKeyword>**](TrackerLanguageKeyword.md)> | The words and phrases defined for the tracker. Each languagueKeywords object contains the words and phrases defined in the specified language. | [optional]
**affiliation** | Option<**String**> | Sets whether to track the keywords when said by people affiliated to a specific company. Options are: \"Anyone\", \"Company\", \"NonCompany\". | [optional]
**part_of_question** | Option<**bool**> | When true, only looks for the keywords when they are part of a question. | [optional]
**said_at** | Option<**String**> | Sets whether the words and phrases are tracked at the beginning, end or anytime in a call. Options are: \"Anytime\", \"First\", or \"Last\". For “First” and “Last” see the saidAtInterval and saidAtUnit fields to see how long to track the keywords for. | [optional]
**said_at_interval** | Option<**i32**> | Sets the period of time to check if the words or phrases came up in the call (according to the saidAt setting). This can be either minutes or percent depending on the value in the saidAtUnit parameter. | [optional]
**said_at_unit** | Option<**String**> | Sets whether the time to look for keywords is in minutes or a percentage of the call duration (according to the saidAt setting). | [optional]
**said_in_topics** | Option<**Vec<String>**> | Sets the topics in the call the tracker terms should be picked up in. | [optional]
**said_in_call_parts** | Option<**Vec<String>**> | Sets the parts of the call to look for the keyword trackers in. | [optional]
**filter_query** | Option<**String**> | A filter that defines which calls to include when searching for the keyword tracker. The filter is in JSON and is in the URL of the search page. Example: a filter for all outbound calls { \"type\": \"And\", \"filters\": [ { \"type\": \"CallDirection\", \"terms\": [\"OUTBOUND\"] } ] } | [optional]
**created** | Option<**String**> | Sets the date and time the tracker was created, in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**creator_user_id** | Option<**String**> | The Id of the team member who created the tracker. Null when the tracker is built-in. | [optional]
**updated** | Option<**String**> | The date and time the tracker settings were last updated, in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**updater_user_id** | Option<**String**> | The Id of the team member who updated the tracker. Null when the tracker is built-in. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


