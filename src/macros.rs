macro_rules! table {
    (
        $name:ident {
            $($column_name:ident -> $Type:ty,)+
        }
    ) => {
        mod $name {
            use {QuerySource, Table};
            use types::*;

            #[allow(non_camel_case_types)]
            pub struct table;

            pub type SqlType = ($($Type),+);

            impl QuerySource for table {
                type SqlType = SqlType;

                fn select_clause(&self) -> String {
                    format!("{}.*", stringify!($name))
                }

                fn from_clause(&self) -> String {
                    stringify!($name).to_string()
                }
            }

            impl Table for table {
                fn name(&self) -> &str {
                    stringify!($name)
                }
            }

            pub mod columns {
                use super::table;
                use {Table, Column};
                use types::*;
            $(
                #[allow(non_camel_case_types, dead_code)]
                pub struct $column_name;

                impl Column<$Type, table> for $column_name {
                    fn name(&self) -> String {
                        format!("{}.{}", table.name(), stringify!($column_name))
                    }
                }
            )+}

            pub use self::columns::*;
        }
    }
}

macro_rules! queriable {
    (
        $Struct:ident {
            $($field_name:ident -> $Type:ty,)+
        }
    ) => {
        impl <ST> Queriable<ST> for $Struct where
            ST: NativeSqlType,
            ($($Type),+): types::FromSql<ST>,
        {
            type Row = ($($Type),+);

            fn build(row: Self::Row) -> Self {
                let ($($field_name),+) = row;
                $Struct {
                    $($field_name: $field_name),+
                }
            }
        }
    }
}

macro_rules! joinable {
    ($child:ident -> $parent:ident ($source:ident = $target:ident)) => {
        impl JoinTo<$parent::table> for $child::table {
            fn join_sql(&self) -> String {
                format!("{} = {}", $child::$source.name(), $parent::$target.name())
            }
        }
    }
}
