offset_string_adjustment(_Time, _Unit, "Z") ->
    0;
offset_string_adjustment(_Time, _Unit, "z") ->
    0;
offset_string_adjustment(_Time, _Unit, Tz) ->
    [Sign, H1, H2 | MinutesDiff] = Tz,
    Hour = list_to_integer([H1, H2]),
    Min = case MinutesDiff of
              [$:, M1, M2] ->
                  list_to_integer([M1, M2]);
              [] ->
                  0
          end,
    Adjustment = 3600 * Hour + 60 * Min,
    case Sign of
        $- -> -Adjustment;
        $+ -> Adjustment
    end.
