# AnsweredScorecard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**answered_scorecard_id** | Option<**i64**> | The identifier of the answer of the scorecard. | [optional]
**scorecard_id** | Option<**i64**> | The identifier of the scorecard. | [optional]
**scorecard_name** | Option<**String**> | Scorecard name. | [optional]
**call_id** | Option<**i64**> | Gong's unique numeric identifier for the call (up to 20 digits). | [optional]
**call_start_time** | Option<**String**> | The date and time when the call was recorded in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**reviewed_user_id** | Option<**i64**> | The user Id of the team member being reviewed, who hosted the call. | [optional]
**reviewer_user_id** | Option<**i64**> | The user Id of the team member who answered the scorecard for the call. | [optional]
**review_time** | Option<**String**> | The date and time when the review was completed and published in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**visibility_type** | Option<**String**> |  | [optional]
**answers** | Option<[**Vec<crate::models::Answer>**](Answer.md)> | The answers in the answered scorecard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


