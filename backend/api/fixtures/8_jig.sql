insert into jig (id, creator_id, author_id, created_at, language, description, is_public)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en', 'test description', true);


insert into jig_module (jig_id, id, index, kind, contents, created_at, is_complete)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc', 0, 0, '{}', '2021-03-04 00:46:26.134651+00', false),
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a', 1, null, null, '2021-03-04 00:46:26.134651+00', false);

insert into jig_additional_resource (jig_id, id, url)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '41b8d0b4-aaff-4942-88ba-1a32fecdbd23', 'url://url.url.url/url'),
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '7a227007-b957-4601-bf15-87a6b86da672', 'url://test.url.testst/s/s');
