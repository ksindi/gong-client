# NewMeetingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**meeting_id** | Option<**String**> | Gong's unique identifier for the meeting (up to 20 digits). | [optional]
**meeting_url** | Option<**String**> | The Gong URL of the meeting, should be used to enter the meeting. | [optional]
**additional_invitees** | Option<[**Vec<crate::models::MeetingInvitee>**](MeetingInvitee.md)> | Attendees the requesting party should add to the invitation, this should support adding email addresses such as coordinator@gong.io for Gong to schedule the recording of the meeting. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


