# Scorecard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scorecard_id** | Option<**i64**> | The identifier of the scorecard. | [optional]
**scorecard_name** | Option<**String**> | Scorecard name. | [optional]
**workspace_id** | Option<**i64**> | Scorecard workspaceId. | [optional]
**enabled** | Option<**bool**> | If the scorecard is enabled or not. | [optional]
**updater_user_id** | Option<**i64**> | The user Id of the team member that updated the scorecard. | [optional]
**created** | Option<**String**> | The date and time when the scorecard was created in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**updated** | Option<**String**> | The date and time when the scorecard was updated in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**questions** | Option<[**Vec<crate::models::Question>**](Question.md)> | The questions in the scorecard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


