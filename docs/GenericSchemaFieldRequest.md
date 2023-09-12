# GenericSchemaFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unique_name** | **String** | The unique name of the field in the CRM system | 
**label** | **String** | The label to use in the UI for this field | 
**r#type** | **String** | The field type (case-sensitive). Must be one of the types listed above. | 
**last_modified** | Option<**String**> | The date and time the schema was last modified. <br>Valid format: ISO-8601 format without milliseconds e.g., \"2020-12-17T13:45:01Z\" | [optional]
**is_deleted** | Option<**bool**> | When true, deletes the field from the schema and its value is removed from all objects. Use with caution | [optional]
**reference_to** | Option<**String**> | The object type this field refers to. Required for field of type REFERENCE. Must be one of \"ACCOUNT\", \"CONTACT\", \"DEAL\", \"LEAD\" or \"USER\" | [optional]
**ordered_value_list** | Option<**Vec<String>**> | The list of values for the field. Required for PICKLIST fields | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


