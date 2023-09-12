# PhoneNumberReferences

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**emails** | Option<[**Vec<crate::models::EmailMessage>**](EmailMessage.md)> | Related email messages. | [optional]
**calls** | Option<[**Vec<crate::models::CallReference>**](CallReference.md)> | Related calls. | [optional]
**meetings** | Option<[**Vec<crate::models::Meeting>**](Meeting.md)> | Related meetings. | [optional]
**customer_data** | Option<[**Vec<crate::models::CustomerData>**](CustomerData.md)> | A list of links to data from external systems (such as CRM, Telephony System, Case Management, etc.) that reference the email address and that Gong stores. | [optional]
**supplied_phone_number** | Option<**String**> | The supplied phone number. | [optional]
**matching_phone_numbers** | Option<**Vec<String>**> | Matching phone numbers found. | [optional]
**email_addresses** | Option<**Vec<String>**> | Related email addresses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


