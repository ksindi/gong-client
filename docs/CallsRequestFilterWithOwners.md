# CallsRequestFilterWithOwners

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_date_time** | Option<**String**> | Date and time (in ISO-8601 format: '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC) from which to list recorded calls. Returns calls that started on or after the specified date and time. If not provided, list starts with earliest call. The date applies to call start time. | [optional]
**to_date_time** | Option<**String**> | Date and time (in ISO-8601 format: '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC) until which to list recorded calls. Returns calls that started up to but excluding specified date and time. If not provided, list ends with most recent call. The date applies to call start time. | [optional]
**workspace_id** | Option<**String**> | Optional Workspace identifier, if supplied the API will return only the calls belonging to this workspace. | [optional]
**call_ids** | Option<**Vec<String>**> | List of calls Ids to be filtered. If not supplied, returns all calls between fromDateTime and toDateTime. | [optional]
**primary_user_ids** | Option<**Vec<String>**> | An optional list of user identifiers, if supplied the API will return only the calls hosted by the specified users. The identifiers in this field match the primaryUserId field of the calls. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


