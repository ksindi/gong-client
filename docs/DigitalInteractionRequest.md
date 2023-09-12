# DigitalInteractionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | The provider's unique identifier for the event used for deduplication | 
**timestamp** | **String** | The date and time of the event in ISO 8601 format | 
**event_type** | **String** | The type of the event, explaining the content and action performed. Values may be partner-aligned, for example, \"link clicked\", \"page viewed\" | 
**source_system_name** | Option<**String**> | When integrating via IPaaS, the name of the technology partner the customer set up a Gong integration with via an IPaaS. When integrating directly with Gong: the name of the company setting up the integration with Gong | [optional]
**session_id** | Option<**String**> | The identifier for the session, useful for tying related events together | [optional]
**device** | Option<**String**> | The device used during the event | [optional]
**content** | [**crate::models::Content**](Content.md) |  | 
**person** | [**crate::models::Person**](Person.md) |  | 
**custom_fields** | Option<[**Vec<crate::models::CustomField>**](CustomField.md)> | Include additional custom data about the event | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


