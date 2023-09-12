# ContentSelector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context** | Option<**String**> | If 'Basic', add links to external systems (context objects) such as CRM, Telephony System, Case Management. If 'Extended' include also data (context fields) for these links. Default value 'None' | [optional]
**context_timing** | Option<**Vec<String>**> | Timing for the context data. The field is optional and can contain either \"Now\" or \"TimeOfCall\" or both. The default value is [\"Now\"]. Can be provided only when the context field is set to 'Extended' | [optional]
**exposed_fields** | Option<[**crate::models::ExposedFields**](ExposedFields.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


