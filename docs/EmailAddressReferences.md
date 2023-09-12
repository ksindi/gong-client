# EmailAddressReferences

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**emails** | Option<[**Vec<crate::models::EmailMessage>**](EmailMessage.md)> | Related email messages. | [optional]
**calls** | Option<[**Vec<crate::models::CallReference>**](CallReference.md)> | Related calls. | [optional]
**meetings** | Option<[**Vec<crate::models::Meeting>**](Meeting.md)> | Related meetings. | [optional]
**customer_data** | Option<[**Vec<crate::models::CustomerData>**](CustomerData.md)> | A list of links to data from external systems (such as CRM, Telephony System, Case Management, etc.) that reference the email address and that Gong stores. | [optional]
**customer_engagement** | Option<[**Vec<crate::models::CustomerEngagement>**](CustomerEngagement.md)> | A list of customer's engagement (such as viewing external share call) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


