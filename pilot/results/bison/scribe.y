%define lr.type lalr
%define parse.error verbose
%token T_A T_ABOUT T_ADDED T_AFTER T_ALL T_AN T_AND T_ANY T_ANYTHING T_AS T_ASSERT T_ASSUMED T_ASSUMES T_AT T_ATTENUATED T_AUTHORITY T_BACK T_BACKOFF T_BE T_BECAUSE T_BEFORE T_BUT T_BY T_CAN T_CANNOT T_CHANGES T_CHECKING T_CHOICE T_CHOOSE T_COMMA T_COMPATIBILITY T_COMPONENT T_COMPONENTS T_CONSUMED T_CONTAINS T_DATABASE T_DATE T_DEADLINE T_DECIMAL T_DECLASSIFIES T_DECREASES T_DEFINE T_DOMAIN T_DOMAINS T_EACH T_EDITABLE T_ELIDED T_ELSE T_ENSURE T_ENTRY T_EQUAL T_EVERY T_EVERYTHING T_EXAMPLE T_EXPECT T_FAIL T_FAILS T_FAILURE T_FIELD T_FILES T_FOLLOWED T_FOR T_FROM T_GIVE T_GIVEN T_GIVES T_HANDLE T_HOLD T_HOLDS T_HOST T_IDEMPOTENT T_IF T_IMPLIES T_IN T_INT T_INTO T_INVARIANT T_IS T_IT T_ITS T_LABELLED T_LEAST T_LEAVES T_LEMMA T_LESS T_LESSTHAN T_LET T_LIMITED T_LPAR T_LSQB T_MATCHING T_MAY T_MAYBE T_MIGRATION T_MINUS T_MOD T_MODEL T_MONITOR T_MORE T_MORETHAN T_MOST T_MUST T_NAME T_NEEDS T_NETWORK T_NO T_NON_EMPTY T_NOT T_NOTHING T_OF T_ON T_ONCE T_ONE T_ONLY T_ONTO T_OPERATIONS T_OR T_OTHERWISE T_OVER T_OWN T_OWNER T_PER T_PIPELINE T_PLUS T_PORT T_POSITIVE T_PRODUCES T_PROOF T_PROPERTY T_PROSE T_PROTOCOL T_PROVIDES T_QUANTITY T_QUEUE T_REACH T_READ T_REMOVED T_REPEAT T_RESERVED T_RESTARTS T_RESULT T_RETRY T_ROOT T_RPAR T_RSQB T_RULES T_RUNS T_SAMPLING T_SCHEMA T_SENDS T_SET T_SKIP T_SLASH T_SOME T_STAGE T_STAGES T_STAR T_STARTS T_STATE T_STOP T_STRING T_SUCCEEDS T_SURVIVES T_SYSTEM T_THAN T_THAT T_THE T_THERE T_THROUGH T_TIER T_TIME T_TIMES T_TO T_TOGETHER T_TRUST T_TRY T_TURNING T_TYPE T_UNDER T_UNTIL T_USE T_USES T_USING T_VALUE T_VAR T_VERSION T_WHEN T_WHERE T_WHILE T_WIRING T_WITH T_WORKFLOW T_WRITE T__DEDENT T__INDENT T__NL T___ANON_0 T___ANON_1 T___ANON_2 T___ANON_3 T___ANON_4 T___ANON_5 T___ANON_6 T___ANON_7 T___ANON_8
%start nt_start
%%
nt_start
    : nt___start_star_0
    | %empty
    ;
nt_item
    : nt_operation
    | nt_workflow
    | nt_define
    | nt_typedecl
    | nt_choice
    | nt_protocol
    | nt_schema
    | nt_migration
    | nt_component
    | nt_system
    | nt_example
    | nt_property
    | nt_stmt
    ;
nt_block
    : T__NL T__INDENT nt___block_plus_1 T__DEDENT
    ;
nt_operation
    : T_TO nt_signature T__NL T__INDENT nt___operation_star_2 nt___block_plus_1 nt_proof T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt___operation_star_2 nt___block_plus_1 T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt___operation_star_2 nt_proof T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt___operation_star_2 T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt___block_plus_1 nt_proof T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt___block_plus_1 T__DEDENT
    | T_TO nt_signature T__NL T__INDENT nt_proof T__DEDENT
    | T_TO nt_signature T__NL T__INDENT T__DEDENT
    | T_TO nt_signature T__NL
    ;
nt_workflow
    : T_WORKFLOW nt_signature T__NL T__INDENT nt___operation_star_2 nt___block_plus_1 T__DEDENT
    | T_WORKFLOW nt_signature T__NL T__INDENT nt___operation_star_2 T__DEDENT
    | T_WORKFLOW nt_signature T__NL T__INDENT nt___block_plus_1 T__DEDENT
    | T_WORKFLOW nt_signature T__NL T__INDENT T__DEDENT
    ;
nt_signature
    : nt_name nt_name nt___signature_star_3
    | nt_name nt_name
    | nt_name nt___signature_star_3
    | nt_name
    ;
nt_clause
    : nt_given
    | nt_produces
    | nt_fails
    | nt_expect
    | nt_ensure
    | nt_changes
    | nt_uses_clause
    | nt_tier
    | nt_idempotent
    | nt_declassifies
    | nt_deadline
    | nt_runs
    | nt_survives
    | nt_holds
    ;
nt_given
    : T_GIVEN nt_name T_IS nt_typeexpr T__NL
    ;
nt_produces
    : T_PRODUCES nt_typeexpr T__NL
    ;
nt_fails
    : T_FAILS T_WITH nt_name_list T__NL
    ;
nt_expect
    : T_EXPECT nt_expr nt_because T__NL
    | T_EXPECT nt_expr T__NL
    ;
nt_ensure
    : T_ENSURE nt_ensure_body nt_because T__NL
    | T_ENSURE nt_ensure_body T__NL
    ;
nt_ensure_body
    : nt_expr
    | T_IF T_IT T_SUCCEEDS T_COMMA nt_expr
    | T_IF T_IT T_FAILS T_WITH T_ANYTHING T_BUT nt_name T_COMMA nt_expr
    | T_IF T_IT T_FAILS T_COMMA nt_expr
    | T_IF nt_expr T_COMMA nt_expr
    ;
nt_changes
    : T_CHANGES T_ONLY nt_expr T__NL
    ;
nt_uses_clause
    : T_USES nt_name_list T__NL
    ;
nt_tier
    : T_TIER nt_name T_FOR nt_name T_COMMA nt_name T_FOR T_EVERYTHING T_ELSE T__NL
    | T_TIER nt_name T_FOR nt_name T__NL
    | T_TIER nt_name T__NL
    ;
nt_idempotent
    : T_IDEMPOTENT T_BY nt_name_list T__NL
    ;
nt_declassifies
    : T_DECLASSIFIES nt_name T_TO nt_name nt_because T__NL
    ;
nt_deadline
    : T_DEADLINE T_QUANTITY T__NL
    ;
nt_runs
    : T_RUNS T_AT T_MOST T_ONCE T_PER nt_name T__NL
    ;
nt_survives
    : T_SURVIVES T_RESTARTS T__NL
    ;
nt_holds
    : T_HOLDS T_WHEN nt_expr T__NL
    ;
nt_because
    : T_BECAUSE T_STRING
    ;
nt_name_list
    : nt_name nt___name_list_star_4
    | nt_name
    ;
nt_typeexpr
    : nt_article nt___typeexpr_star_5 nt_type_name nt___typeexpr_star_6
    | nt_article nt___typeexpr_star_5 nt_type_name
    | nt_article nt_type_name nt___typeexpr_star_6
    | nt_article nt_type_name
    | nt___typeexpr_star_5 nt_type_name nt___typeexpr_star_6
    | nt___typeexpr_star_5 nt_type_name
    | nt_type_name nt___typeexpr_star_6
    | nt_type_name
    ;
nt_article
    : T_A
    | T_AN
    | T_THE
    ;
nt_modifier
    : T_POSITIVE
    | T_EDITABLE
    | T_CONSUMED
    | T_MAYBE
    | T_NON_EMPTY
    ;
nt_type_name
    : nt_name T_OF nt___typeexpr_star_5 nt_type_name
    | nt_name T_OF nt_type_name
    | nt_name
    ;
nt_type_suffix
    : T_WITH nt_name nt_range
    | T_LIMITED T_TO nt_cap_spec
    | T_LABELLED nt_name
    | T_IN T_STATE nt_name nt___type_suffix_star_7
    | T_IN T_STATE nt_name
    | T_AS nt_name
    | T_MATCHING nt_name
    | T_LESS T_THAN nt_expr
    | T_AT T_MOST T_QUANTITY
    ;
nt_range
    : nt_expr nt_range_word nt_expr
    ;
nt_range_word
    : T_UNTIL
    | T_THROUGH
    ;
nt_cap_spec
    : nt_cap_kind nt___cap_spec_plus_8
    | nt___cap_spec_plus_8
    ;
nt_cap_kind
    : T_FILES
    | T_NETWORK
    | T_DATABASE
    ;
nt_cap_part
    : T_UNDER T_STRING
    | T_HOST T_STRING
    | T_PORT T_INT
    | T_STRING
    | T_READ T_ONLY
    | T_READ T_AND T_WRITE
    ;
nt_typedecl
    : T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T__NL
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T_IN T_STATE nt_name T__NL
    | T_TYPE nt_name T_OF nt_name T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T_OF nt_name T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_OF nt_name T__NL
    | T_TYPE nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_IN T_STATE nt_name nt___type_suffix_star_7 T__NL
    | T_TYPE nt_name T_IN T_STATE nt_name T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_IN T_STATE nt_name T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T_IN T_STATE nt_name T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_IN T_STATE nt_name T__NL
    | T_TYPE nt_name T_IS nt_typeexpr T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T_IS nt_typeexpr T__NL
    | T_TYPE nt_name T__NL T__INDENT nt___typedecl_plus_9 T__DEDENT
    | T_TYPE nt_name T__NL
    ;
nt_member
    : T_FIELD nt_name T_AS nt_typeexpr T__NL
    | T_MODEL nt_name T_AS nt_expr T__NL
    | T_INVARIANT nt_expr nt_because T__NL
    | T_INVARIANT nt_expr T__NL
    ;
nt_choice
    : T_CHOICE nt_name T__NL T__INDENT nt___choice_plus_10 T__DEDENT
    ;
nt_variant
    : nt_name T_WITH nt_field_decl nt___variant_star_11 T__NL
    | nt_name T_WITH nt_field_decl T__NL
    | nt_name T__NL
    ;
nt_field_decl
    : nt_name T_AS nt_typeexpr
    ;
nt_define
    : T_DEFINE nt_name T_IS nt_predicate_phrase T__NL T__INDENT nt___operation_star_2 T__DEDENT
    ;
nt_predicate_phrase
    : nt_name
    | nt_article nt_name T_OF nt_name
    | nt_article nt_name
    ;
nt_example
    : T_EXAMPLE nt_expr T__NL
    ;
nt_property
    : T_PROPERTY nt_quantified T__NL
    ;
nt_protocol
    : T_PROTOCOL nt_name T__NL T__INDENT nt___protocol_plus_12 T__DEDENT
    ;
nt_proto_step
    : nt_role_send
    | T_REPEAT T__NL T__INDENT nt___protocol_plus_12 T__DEDENT T_UNTIL nt_role_send
    ;
nt_role_send
    : nt_name T_SENDS nt_message_spec T__NL
    | nt_name T_SENDS T_ONE T_OF nt_message_spec nt___role_send_plus_13 T__NL
    ;
nt_message_spec
    : nt_name T_WITH nt_field_decl nt___variant_star_11
    | nt_name T_WITH nt_field_decl
    | nt_name
    ;
nt_component
    : T_COMPONENT nt_name T___ANON_0 nt_version T__NL T__INDENT nt___component_plus_14 T__DEDENT
    ;
nt_comp_section
    : T_ABOUT T_PROSE T__NL
    | T_NEEDS T__NL T__INDENT nt___comp_section_plus_15 T__DEDENT
    | T_MAY T__NL T__INDENT nt___comp_section_plus_16 T__DEDENT
    | T_ASSUMES T__NL T__INDENT nt___comp_section_plus_17 T__DEDENT
    | T_PROVIDES T__NL T__INDENT nt___comp_section_plus_18 T__DEDENT
    | T_FAILURE T__NL T__INDENT nt___comp_section_plus_19 T__DEDENT
    | nt_tier
    ;
nt_need
    : nt_name T_FROM nt_version T_AS nt_name T__NL T__INDENT T_USES nt_name_list T__NL T__DEDENT
    | nt_name T_FROM nt_version T_AS nt_name T__NL
    ;
nt_may_line
    : T_NOTHING T_OF T_ITS T_OWN T__NL
    | nt_cap_spec T__NL
    ;
nt_assumption
    : nt_expr T_ASSUMED T_OWNER nt_name T_MONITOR nt_monitor_policy T__NL
    ;
nt_monitor_policy
    : T_CHECKING
    | T_SAMPLING T_QUANTITY
    ;
nt_failure_line
    : T_DOMAIN nt_name T__NL
    | T_RETRY nt_name T_AT T_MOST T_INT T_TIMES T_WITH T_BACKOFF T_QUANTITY nt_because T__NL
    ;
nt_system
    : T_SYSTEM nt_name T___ANON_0 nt_version T__NL T__INDENT nt___system_plus_20 T__DEDENT
    ;
nt_sys_section
    : T_COMPONENTS T__NL T__INDENT nt___sys_section_plus_21 T__DEDENT
    | T_AUTHORITY T__NL T__INDENT nt___sys_section_plus_22 T__DEDENT
    | T_WIRING T__NL T__INDENT nt___sys_section_plus_23 T__DEDENT
    | T_FAILURE T_DOMAINS T__NL T__INDENT nt___sys_section_plus_24 T__DEDENT
    | T_RULES T__NL T__INDENT nt___sys_section_plus_25 T__DEDENT
    ;
nt_authority_line
    : T_ROOT T_GIVES nt_name nt_cap_spec T__NL
    | T_NOTHING T_ELSE T_HOLDS T_AUTHORITY T__NL
    ;
nt_wiring_line
    : nt_name T_NEEDS nt_name_list T_LPAR T_READ T_OPERATIONS T_ONLY T_RPAR T__NL
    | nt_name T_NEEDS nt_name_list T__NL
    ;
nt_domain_line
    : nt_name T_CONTAINS nt_name_list T__NL
    | T_A T_FAILURE T_IN nt_name T_MUST T_NOT T_STOP nt_name T__NL
    ;
nt_rule_line
    : T_ONLY nt_name T_CAN T_REACH nt_cap_spec T__NL
    | T_NO T_VALUE T_LABELLED nt_name T_LEAVES nt_name T__NL
    | nt_name T_CANNOT T_WRITE T_TO nt_cap_spec T__NL
    ;
nt_schema
    : T_SCHEMA nt_name T___ANON_0 T_INT T_COMPATIBILITY nt_name T__NL T__INDENT nt___schema_plus_26 T__DEDENT
    ;
nt_schema_field
    : nt_name T_AS nt_typeexpr T_ADDED T_IN T___ANON_0 T_INT T__NL
    | nt_name T_AS nt_typeexpr T__NL
    | nt_name T_RESERVED T_REMOVED T_IN T___ANON_0 T_INT T__NL
    ;
nt_migration
    : T_MIGRATION nt_name T_FROM T___ANON_0 T_INT T_TO T___ANON_0 T_INT T_TURNING nt_name T_INTO nt_name T__NL T__INDENT nt___migration_plus_27 T__DEDENT
    ;
nt_stmt
    : nt_let
    | nt_var
    | nt_set
    | nt_give_back
    | nt_fail
    | nt_if_stmt
    | nt_for_each
    | nt_repeat_while
    | nt_invariant
    | nt_decreases
    | nt_choose
    | nt_handle
    | nt_hold
    | nt_together
    | nt_pipeline
    | nt_stop
    | nt_skip
    | nt_trust
    | nt_elided
    | nt_expr_stmt
    ;
nt_let
    : T_LET nt_name T_BE nt_expr T_COMMA T_OR T_ELSE nt_expr T__NL
    | T_LET nt_name T_BE nt_expr T__NL
    ;
nt_var
    : T_VAR nt_name T_STARTS T_AT nt_expr T__NL
    ;
nt_set
    : T_SET nt_place T_TO nt_expr T__NL
    ;
nt_place
    : nt_name T_OF nt_place
    | nt_name
    ;
nt_give_back
    : T_GIVE T_BACK nt_expr T__NL
    ;
nt_fail
    : T_FAIL T_WITH nt_name T__NL
    ;
nt_if_stmt
    : T_IF nt_expr nt_block nt___if_stmt_star_28 T_OTHERWISE nt_block
    | T_IF nt_expr nt_block nt___if_stmt_star_28
    | T_IF nt_expr nt_block T_OTHERWISE nt_block
    | T_IF nt_expr nt_block
    ;
nt_for_each
    : T_FOR T_EACH nt_name T_FROM nt_expr nt_range_word nt_expr nt_block
    | T_FOR T_EACH nt_name T_IN nt_expr nt_block
    ;
nt_repeat_while
    : T_REPEAT T_WHILE nt_expr nt_block
    ;
nt_invariant
    : T_INVARIANT nt_expr nt_because T__NL
    | T_INVARIANT nt_expr T__NL
    ;
nt_decreases
    : T_DECREASES nt_expr T__NL
    ;
nt_choose
    : T_CHOOSE nt_expr T__NL T__INDENT nt___choose_plus_29 T__DEDENT
    ;
nt_when
    : T_WHEN nt_pattern nt_block
    ;
nt_pattern
    : T_SOME nt_name
    | T_NOTHING
    | nt_name T_WITH nt_name nt___pattern_star_30
    | nt_name T_WITH nt_name
    | nt_name
    ;
nt_handle
    : T_HANDLE nt_block nt___handle_plus_31
    ;
nt_on_failure
    : T_ON T_FAILURE nt_name nt___type_suffix_star_7 T_AS nt_name nt_block
    | T_ON T_FAILURE nt_name nt___type_suffix_star_7 nt_block
    | T_ON T_FAILURE nt_name T_AS nt_name nt_block
    | T_ON T_FAILURE nt_name nt_block
    | T_ON T_FAILURE T_IN T_ANY T_STAGE nt_block
    ;
nt_hold
    : T_HOLD nt_name T_AS nt_expr nt_block
    ;
nt_together
    : T_TOGETHER nt_block
    ;
nt_pipeline
    : T_PIPELINE T__NL T__INDENT nt___pipeline_plus_32 T__DEDENT nt_on_failure
    | T_PIPELINE T__NL T__INDENT nt___pipeline_plus_32 T__DEDENT
    ;
nt_pipe_item
    : T_STAGE nt_expr T_COMMA T_INT T_AT T_A T_TIME T__NL
    | T_STAGE nt_expr T__NL
    | T_QUEUE T_OF T_AT T_MOST T_QUANTITY T__NL
    ;
nt_stop
    : T_STOP T_ALL T_STAGES T__NL
    | T_STOP T__NL
    ;
nt_skip
    : T_SKIP T__NL
    ;
nt_trust
    : T_TRUST T_STRING T_OWNER nt_name T__NL
    ;
nt_proof
    : T_PROOF T__NL T__INDENT nt___proof_plus_33 T__DEDENT
    ;
nt_proof_stmt
    : T_USE T_LEMMA nt_name T__NL
    | T_ASSERT nt_expr T__NL
    ;
nt_elided
    : T_ELIDED T__NL
    ;
nt_expr_stmt
    : nt_expr T__NL
    ;
nt_expr
    : nt_logic T_IMPLIES nt_logic
    | nt_logic
    ;
nt_logic
    : nt_negation nt___logic_plus_34
    | nt_negation nt___logic_plus_35
    | nt_negation
    ;
nt_negation
    : T_NOT nt_negation
    | nt_quantified
    | nt_comparison
    ;
nt_quantified
    : T_FOR T_EVERY nt_name T_IN nt_sum T_COMMA nt_expr
    | T_FOR T_EVERY nt_name T_FROM nt_sum nt_range_word nt_sum T_COMMA nt_expr
    | T_FOR T_EVERY nt_name T_THAT T_IS nt_typeexpr T_COMMA nt_expr
    | T_FOR T_EVERY nt_name T_COMMA nt_expr
    | T_THERE T_IS nt_article nt_name T_IN nt_sum T_WHERE nt_where_cond
    | T_THERE T_IS nt_name T_IN nt_sum T_WHERE nt_where_cond
    ;
nt_comparison
    : nt_concat
    | nt_concat nt___comparison_plus_36
    | nt_concat T_IS nt_article nt_name T_OF nt_concat
    ;
nt_comp_op
    : T_IS
    | T_EQUAL
    | T___ANON_1
    | T_IS T_NOT
    | T___ANON_2
    | T___ANON_3
    | T_IS T_LESS T_THAN
    | T_LESSTHAN
    | T_IS T_MORE T_THAN
    | T_MORETHAN
    | T_IS T_AT T_MOST
    | T___ANON_4
    | T___ANON_5
    | T_IS T_AT T_LEAST
    | T___ANON_6
    | T___ANON_7
    | T_IS T_IN
    | T___ANON_8
    | T_IS T_NOT T_IN
    | T_IS T_AFTER
    | T_IS T_NOT T_AFTER
    | T_IS T_BEFORE
    ;
nt_concat
    : nt_sum nt___concat_star_37
    | nt_sum
    ;
nt_sum
    : nt_product nt___sum_star_38
    | nt_product
    ;
nt_product
    : nt_unary nt___product_star_39
    | nt_unary
    ;
nt_unary
    : T_MINUS nt_unary
    | nt_postfix
    ;
nt_postfix
    : nt_application
    | nt_application T_AT T_ENTRY
    | nt_application T_ATTENUATED T_TO nt_cap_spec
    ;
nt_application
    : nt_call
    | nt_operand
    | nt_construction
    | T_TRY nt_application
    ;
nt_call
    : nt_name nt___call_star_40
    | nt_name nt___call_star_40 T_OF nt_of_operand nt___call_star_41
    | nt_name nt___call_star_40 T_OF nt_of_operand
    | nt_name T_OF nt_of_operand nt___call_star_41
    | nt_name T_OF nt_of_operand
    | nt_name nt___call_star_40 nt___call_star_41
    | nt_name nt___call_star_41
    | nt_name nt___call_star_40 T_OF nt_comprehension
    | nt_name T_OF nt_comprehension
    ;
nt_label_arg
    : nt_label nt_of_operand
    | T_WHERE nt_where_cond
    ;
nt_of_operand
    : nt_name nt___call_star_40 T_OF nt_of_operand
    | nt_name nt___call_star_40
    | nt_name T_OF nt_of_operand
    | nt_name
    | nt_atom
    ;
nt_comprehension
    : nt_of_operand T_FOR T_EACH nt_name T_IN nt_of_operand T_WHERE nt_where_cond
    | nt_of_operand T_FOR T_EACH nt_name T_IN nt_of_operand
    ;
nt_where_cond
    : nt_expr
    ;
nt_label
    : T_FROM
    | T_FOR
    | T_WITH
    | T_IN
    | T_INTO
    | T_ONTO
    | T_OVER
    | T_USING
    | T_TO
    | T_BY
    | T_AS
    ;
nt_construction
    : nt_article nt_name T_FROM nt_operand nt___construction_star_42 T_WITH nt_name nt_operand
    | nt_article nt_name T_FROM nt_operand nt___construction_star_42
    | nt_article nt_name T_FROM nt_operand T_WITH nt_name nt_operand
    | nt_article nt_name T_FROM nt_operand
    ;
nt_operand
    : nt_name
    | nt_atom
    ;
nt_atom
    : nt_literal
    | T_LPAR nt_expr T_RPAR
    | T_LPAR nt_comprehension T_RPAR
    | T_LSQB nt_expr nt___atom_star_43 T_RSQB
    | T_LSQB nt_expr T_RSQB
    | T_LSQB T_RSQB
    | T_RESULT
    | T_NOTHING
    | T_SOME nt_operand
    ;
nt_literal
    : T_INT
    | T_DECIMAL
    | T_VERSION
    | T_STRING
    | T_QUANTITY
    | T_DATE
    ;
nt_version
    : T_VERSION
    | T_DECIMAL
    | T_INT
    ;
nt_name
    : T_NAME
    | T_ABOUT
    | T_ADDED
    | T_ALL
    | T_ANY
    | T_ANYTHING
    | T_ASSERT
    | T_ASSUMED
    | T_ASSUMES
    | T_AUTHORITY
    | T_BACKOFF
    | T_BUT
    | T_CAN
    | T_CANNOT
    | T_CHECKING
    | T_COMPATIBILITY
    | T_COMPONENTS
    | T_CONSUMED
    | T_CONTAINS
    | T_DATABASE
    | T_DOMAIN
    | T_DOMAINS
    | T_EDITABLE
    | T_ELSE
    | T_ENTRY
    | T_EVERYTHING
    | T_FILES
    | T_GIVES
    | T_HOST
    | T_ITS
    | T_LABELLED
    | T_LEAVES
    | T_LEMMA
    | T_LIMITED
    | T_MATCHING
    | T_MAY
    | T_MAYBE
    | T_MONITOR
    | T_MUST
    | T_NEEDS
    | T_NETWORK
    | T_NO
    | T_NON_EMPTY
    | T_ONCE
    | T_ONE
    | T_ONLY
    | T_OPERATIONS
    | T_OWN
    | T_OWNER
    | T_PER
    | T_PORT
    | T_POSITIVE
    | T_PROVIDES
    | T_QUEUE
    | T_REACH
    | T_READ
    | T_REMOVED
    | T_RESERVED
    | T_RESTARTS
    | T_RETRY
    | T_ROOT
    | T_RULES
    | T_SAMPLING
    | T_SENDS
    | T_STAGE
    | T_STAGES
    | T_STATE
    | T_SUCCEEDS
    | T_THAT
    | T_TIME
    | T_TIMES
    | T_TURNING
    | T_UNDER
    | T_USE
    | T_VALUE
    | T___ANON_0
    | T_WIRING
    | T_WRITE
    ;
nt___start_star_0
    : nt_item
    | nt___start_star_0 nt_item
    ;
nt___block_plus_1
    : nt_stmt
    | nt___block_plus_1 nt_stmt
    ;
nt___operation_star_2
    : nt_clause
    | nt___operation_star_2 nt_clause
    ;
nt___signature_star_3
    : nt_label nt_name
    | T_OF nt_name
    | nt___signature_star_3 nt_label nt_name
    | nt___signature_star_3 T_OF nt_name
    ;
nt___name_list_star_4
    : T_COMMA nt_name
    | nt___name_list_star_4 T_COMMA nt_name
    ;
nt___typeexpr_star_5
    : nt_modifier
    | nt___typeexpr_star_5 nt_modifier
    ;
nt___typeexpr_star_6
    : nt_type_suffix
    | nt___typeexpr_star_6 nt_type_suffix
    ;
nt___type_suffix_star_7
    : T_OR nt_name
    | nt___type_suffix_star_7 T_OR nt_name
    ;
nt___cap_spec_plus_8
    : nt_cap_part
    | nt___cap_spec_plus_8 nt_cap_part
    ;
nt___typedecl_plus_9
    : nt_member
    | nt___typedecl_plus_9 nt_member
    ;
nt___choice_plus_10
    : nt_variant
    | nt___choice_plus_10 nt_variant
    ;
nt___variant_star_11
    : T_AND nt_field_decl
    | nt___variant_star_11 T_AND nt_field_decl
    ;
nt___protocol_plus_12
    : nt_proto_step
    | nt___protocol_plus_12 nt_proto_step
    ;
nt___role_send_plus_13
    : T_COMMA T_OR nt_message_spec
    | nt___role_send_plus_13 T_COMMA T_OR nt_message_spec
    ;
nt___component_plus_14
    : nt_comp_section
    | nt___component_plus_14 nt_comp_section
    ;
nt___comp_section_plus_15
    : nt_need
    | nt___comp_section_plus_15 nt_need
    ;
nt___comp_section_plus_16
    : nt_may_line
    | nt___comp_section_plus_16 nt_may_line
    ;
nt___comp_section_plus_17
    : nt_assumption
    | nt___comp_section_plus_17 nt_assumption
    ;
nt___comp_section_plus_18
    : nt_operation
    | nt___comp_section_plus_18 nt_operation
    ;
nt___comp_section_plus_19
    : nt_failure_line
    | nt___comp_section_plus_19 nt_failure_line
    ;
nt___system_plus_20
    : nt_sys_section
    | nt___system_plus_20 nt_sys_section
    ;
nt___sys_section_plus_21
    : nt_name nt_version T__NL
    | nt___sys_section_plus_21 nt_name nt_version T__NL
    ;
nt___sys_section_plus_22
    : nt_authority_line
    | nt___sys_section_plus_22 nt_authority_line
    ;
nt___sys_section_plus_23
    : nt_wiring_line
    | nt___sys_section_plus_23 nt_wiring_line
    ;
nt___sys_section_plus_24
    : nt_domain_line
    | nt___sys_section_plus_24 nt_domain_line
    ;
nt___sys_section_plus_25
    : nt_rule_line
    | nt___sys_section_plus_25 nt_rule_line
    ;
nt___schema_plus_26
    : nt_schema_field
    | nt___schema_plus_26 nt_schema_field
    ;
nt___migration_plus_27
    : nt_ensure
    | nt___migration_plus_27 nt_ensure
    ;
nt___if_stmt_star_28
    : T_OTHERWISE T_IF nt_expr nt_block
    | nt___if_stmt_star_28 T_OTHERWISE T_IF nt_expr nt_block
    ;
nt___choose_plus_29
    : nt_when
    | nt___choose_plus_29 nt_when
    ;
nt___pattern_star_30
    : T_AND nt_name
    | nt___pattern_star_30 T_AND nt_name
    ;
nt___handle_plus_31
    : nt_on_failure
    | nt___handle_plus_31 nt_on_failure
    ;
nt___pipeline_plus_32
    : nt_pipe_item
    | nt___pipeline_plus_32 nt_pipe_item
    ;
nt___proof_plus_33
    : nt_proof_stmt
    | nt___proof_plus_33 nt_proof_stmt
    ;
nt___logic_plus_34
    : T_AND nt_negation
    | nt___logic_plus_34 T_AND nt_negation
    ;
nt___logic_plus_35
    : T_OR nt_negation
    | nt___logic_plus_35 T_OR nt_negation
    ;
nt___comparison_plus_36
    : nt_comp_op nt_concat
    | nt___comparison_plus_36 nt_comp_op nt_concat
    ;
nt___concat_star_37
    : T_FOLLOWED T_BY nt_sum
    | nt___concat_star_37 T_FOLLOWED T_BY nt_sum
    ;
nt___sum_star_38
    : T_PLUS nt_product
    | T_MINUS nt_product
    | nt___sum_star_38 T_PLUS nt_product
    | nt___sum_star_38 T_MINUS nt_product
    ;
nt___product_star_39
    : T_STAR nt_unary
    | T_SLASH nt_unary
    | T_MOD nt_unary
    | nt___product_star_39 T_STAR nt_unary
    | nt___product_star_39 T_SLASH nt_unary
    | nt___product_star_39 T_MOD nt_unary
    ;
nt___call_star_40
    : nt_operand
    | nt___call_star_40 nt_operand
    ;
nt___call_star_41
    : nt_label_arg
    | nt___call_star_41 nt_label_arg
    ;
nt___construction_star_42
    : T_COMMA nt_operand
    | nt___construction_star_42 T_COMMA nt_operand
    ;
nt___atom_star_43
    : T_COMMA nt_expr
    | nt___atom_star_43 T_COMMA nt_expr
    ;
%%
