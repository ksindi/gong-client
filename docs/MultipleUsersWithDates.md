# MultipleUsersWithDates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_date** | [**String**](string.md) | The date (in the company's time zone) from which to list a user's activity. This value is inclusive. The date should be specified in the following format: YYYY-MM-DD. | 
**to_date** | [**String**](string.md) | The date (in the company's time zone) until which to list a user's activity. This value is exclusive. This value should not exceed the current day.The date should be specified in the following format: YYYY-MM-DD. | 
**created_from_date_time** | Option<**String**> |  | [optional]
**created_to_date_time** | Option<**String**> |  | [optional]
**user_ids** | Option<**Vec<String>**> | Set of Gong's unique numeric identifiers for the users (up to 20 digits). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


