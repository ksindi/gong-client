# UpdateMeetingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_time** | **String** | The meeting start time in ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | 
**end_time** | **String** | The meeting end time in ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | 
**title** | Option<**String**> | Title of the event. | [optional]
**invitees** | [**Vec<crate::models::MeetingInvitee>**](MeetingInvitee.md) | A list of email addresses of invitees to the event (not including the organizer). | 
**external_id** | Option<**String**> | The ID as it is formed on the external system. | [optional]
**organizer_email** | **String** | The email address of the user who created the meeting. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


