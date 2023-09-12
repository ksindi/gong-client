# CompanyUsersAggregateActivityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**records** | Option<[**crate::models::Records**](Records.md)> |  | [optional]
**users_aggregate_activity_stats** | Option<[**Vec<crate::models::UserActivity>**](UserActivity.md)> | A list, in which each item specifies one user's activities. | [optional]
**from_date_time** | Option<**String**> | The date and time in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC), when the list of results starts. | [optional]
**to_date_time** | Option<**String**> | The date and time in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC), when the list of results ends. | [optional]
**time_zone** | Option<**String**> | The company's defined timezone in Gong. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


