use rusqlite::{
    ffi,
    vtab::{
        self, eponymous_only_module, CreateVTab, IndexConstraintOp, IndexInfo, VTab, VTabCursor,
        VTabKind,
    },
    Connection, Error, Result,
};
use std::os::raw::c_int;

#[repr(C)]
struct QueryEachTable {
    base: ffi::sqlite3_vtab,
}

unsafe impl<'vtab> VTab<'vtab> for QueryEachTable {
    type Aux = ();
    type Cursor = QueryEachCursor;

    fn connect(
        _db: &mut vtab::VTabConnection,
        _aux: Option<&Self::Aux>,
        _args: &[&[u8]],
    ) -> Result<(String, Self)> {
        let schema = "CREATE TABLE x(query hidden, name text, value text)";
        Ok((
            schema.to_string(),
            QueryEachTable {
                base: ffi::sqlite3_vtab::default(),
            },
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
        let relevant_constraints: Vec<(usize, i32)> = info
            .constraints()
            .enumerate()
            .filter_map(|(i, constraint)| {
                if constraint.column() == 0
                    && constraint.is_usable()
                    && constraint.operator() == IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
                {
                    Some((i, 1))
                } else {
                    None
                }
            })
            .collect();

        for (index, argv_index) in relevant_constraints {
            let mut usage = info.constraint_usage(index);
            usage.set_argv_index(argv_index);
            usage.set_omit(true);
        }

        info.set_estimated_cost(1.0);

        Ok(())
    }

    fn open(&mut self) -> Result<Self::Cursor> {
        Ok(QueryEachCursor {
            rows: vec![],
            index: 0,
            base: ffi::sqlite3_vtab_cursor::default(),
        })
    }
}

impl CreateVTab<'_> for QueryEachTable {
    const KIND: VTabKind = VTabKind::EponymousOnly;
}

#[repr(C)]
struct QueryEachCursor {
    base: ffi::sqlite3_vtab_cursor,
    rows: Vec<(String, String)>,
    index: usize,
}

unsafe impl VTabCursor for QueryEachCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        args: &vtab::Values<'_>,
    ) -> Result<()> {
        if args.is_empty() {
            return Err(Error::ModuleError(
                "Missing required query argument.".to_string(),
            ));
        }

        let query = args.get::<String>(0)?.to_string();
        self.rows = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();
        self.index = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.index += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.index >= self.rows.len()
    }

    fn column(&self, ctx: &mut vtab::Context, col: i32) -> Result<()> {
        match col {
            1 => ctx.set_result(&self.rows[self.index].0),
            2 => ctx.set_result(&self.rows[self.index].1),
            _ => Ok(()),
        }
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.index as i64)
    }
}

pub fn register_query_each_virtual_table(conn: &Connection) -> Result<()> {
    conn.create_module(
        "url_query_each",
        eponymous_only_module::<QueryEachTable>(),
        None,
    )
}

