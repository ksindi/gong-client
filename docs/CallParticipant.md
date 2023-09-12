# CallParticipant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<**String**> | The phone number of the party, if available. | [optional]
**email_address** | Option<**String**> | The email address of the party, if available. | [optional]
**name** | Option<**String**> | The name of the party, if available. | [optional]
**party_id** | Option<**String**> | An identifier that is only required when speakersTimeline is provided.  The partyId is used to recognize the speakers within the provided speakersTimeline. | [optional]
**media_channel_id** | Option<**i32**> | The audio channel corresponding to the company team member (rep) used when the uploaded media file is multi-channel (stereo). The channel id is either 0 or 1 (representing left or right respectively) | [optional]
**context** | Option<[**Vec<crate::models::PartyUploadContext>**](PartyUploadContext.md)> | A list of links to external systems such as CRM, Telephony System, Case Management, etc. | [optional]
**user_id** | Option<**String**> | The user ID of the participant within the Gong system, if the participant is a user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


