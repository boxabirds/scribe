    def _handle_rest(self, st, line, fpname):
        # a section header or option header?
        if self._allow_unnamed_section and st.cursect is None:
            self._handle_header(st, UNNAMED_SECTION, fpname)

        st.indent_level = st.cur_indent_level
        # is it a section header?
        mo = self.SECTCRE.match(line.clean)

        if not mo and st.cursect is None:
            raise MissingSectionHeaderError(fpname, st.lineno, line)

        self._handle_header(st, mo.group('header'), fpname) if mo else self._handle_option(st, line, fpname)
