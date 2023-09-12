# CommentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier of the comment within the Gong's system. | [optional]
**audio_start_time** | Option<**f32**> | The number of seconds from the beginning of the call that the comment start refers to. | [optional]
**audio_end_time** | Option<**f32**> | The number of seconds from the beginning of the call that the comment ends refers to. | [optional]
**commenter_user_id** | Option<**String**> | Unique identifier of the user who wrote the comment. | [optional]
**comment** | Option<**String**> | The comment itself. The comment may contains person tagging in this format @[user name](user Id) | [optional]
**posted** | Option<**String**> | The date and time in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); in which the comment was posted. | [optional]
**in_reply_to** | Option<**String**> | The unique identifier of the original comment in case the current comment is a reply to the original one. | [optional]
**during_call** | Option<**bool**> | True if the comment was written during the call, false if it was posted after the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


