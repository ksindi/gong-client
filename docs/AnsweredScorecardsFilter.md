# AnsweredScorecardsFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**call_from_date** | Option<**String**> | The date (in the company's time zone) of the call from when to list calls. This value is inclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it starts with the earliest recorded call. | [optional]
**call_to_date** | Option<**String**> | The date (in the company's time zone) of the call until when to list calls. This value is exclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it ends with the latest recorded call. | [optional]
**review_from_date** | Option<**String**> | The review date (in the company's time zone) from when to list calls. This value is inclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it starts with the earliest reviewed call. | [optional]
**review_to_date** | Option<**String**> | The review date (in the company's time zone) until when to list calls. This value is exclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it ends with the latest reviewed call. | [optional]
**reviewed_user_ids** | Option<**Vec<String>**> | List of user Ids of the team members being reviewed. If not specified, it returns all answered scorecards in the corresponding filter. | [optional]
**scorecard_ids** | Option<**Vec<String>**> | List of scorecards Ids. If not specified, it returns all answered scorecards in the corresponding filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


