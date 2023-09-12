# Party

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID of the participant in the call. | [optional]
**email_address** | Option<**String**> | Email address. | [optional]
**name** | Option<**String**> | The name of the participant. | [optional]
**title** | Option<**String**> | The job title of the participant | [optional]
**user_id** | Option<**String**> | The user ID of the participant within the Gong system, if the participant exists in the system. | [optional]
**speaker_id** | Option<**String**> | Unique ID of a participant that spoke in the call. References to this id will appear in the '/v2/calls/transcript' endpoint response. | [optional]
**context** | Option<[**Vec<crate::models::PartyContextDto>**](PartyContextDto.md)> | A list of links to external systems such as CRM, Dialer, Case Management, etc. | [optional]
**affiliation** | Option<**String**> | Whether the participant is from the company or not. | [optional]
**phone_number** | Option<**String**> | The phone number of the participant. | [optional]
**methods** | Option<**Vec<String>**> | Whether the participant was invited to the meeting or only attended the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


