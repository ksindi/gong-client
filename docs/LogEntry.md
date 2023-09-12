# LogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Gong's unique numeric identifier for the user (up to 20 digits), if available. | [optional]
**user_email_address** | Option<**String**> | The email address of the user, if available. | [optional]
**user_full_name** | Option<**String**> | The full name of the user, if available. | [optional]
**impersonator_user_id** | Option<**String**> | Gong's unique numeric identifier for the impersonating user (up to 20 digits), if available. | [optional]
**impersonator_email_address** | Option<**String**> | The email address of the impersonating user, if available. | [optional]
**impersonator_full_name** | Option<**String**> | The full name of the impersonating user, if available. | [optional]
**impersonator_company_id** | Option<**String**> | Gong's unique numeric identifier for the impersonating user's company id (up to 20 digits), if available. | [optional]
**event_time** | Option<**String**> | The time in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); when log was created. | [optional]
**log_record** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The list of log fields and associated values. This element will be populated dynamically by the log record, depending on the type of log. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


