use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::ID))
                    .col(string_len(Users::Name, 255).not_null())
                    .col(string_len(Users::Email, 510).not_null().unique_key())
                    .col(string(Users::Photo).not_null().default("default.png"))
                    .col(boolean(Users::Verified).not_null().default(false))
                    .col(string_len(Users::Password, 510).not_null())
                    .col(string_len(Users::Role, 50).not_null().default("user"))
                    .col(date_time(Users::CreatedAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .col(date_time(Users::UpdatedAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_email_idx")
                    .table(Users::Table)
                    .col(Users::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bakery::Table)
                    .if_not_exists()
                    .col(pk_auto(Bakery::ID))
                    .col(string_len(Bakery::Title, 255))
                    .col(string(Bakery::Image))
                    .col(string(Bakery::Details))
                    .col(integer(Bakery::InStocks).not_null().default(0))
                    .col(float(Bakery::Price).not_null().default(0.0))
                    .col(date_time(Bakery::CreatedAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .col(date_time(Bakery::RestockAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .col(pk_auto(Customers::ID))
                    .col(string_len(Customers::Name, 255))
                    .col(string_len(Customers::LastName, 255))
                    .col(string_len(Customers::Email, 510))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Purchase::Table)
                    .col(pk_auto(Purchase::ID))
                    .col(integer(Purchase::CustomerID))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Purchase::Table, Purchase::CustomerID)
                            .to(Customers::Table, Customers::ID),
                    )
                    .col(float(Purchase::SumPrice).not_null().default(0.0))
                    .col(date_time(Purchase::CreatedAt).default(Expr::cust("CURRENT_TIMESTAMP")))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PurchaseBakery::Table)
                    .col(pk_auto(PurchaseBakery::ID))
                    .col(integer(PurchaseBakery::PurchaseID))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PurchaseBakery::Table, PurchaseBakery::PurchaseID)
                            .to(Purchase::Table, Purchase::ID),
                    )
                    .col(integer(PurchaseBakery::BakeryID))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PurchaseBakery::Table, PurchaseBakery::BakeryID)
                            .to(Bakery::Table, Bakery::ID),
                    )
                    .col(integer(PurchaseBakery::Quantity).not_null().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(
                Index::drop().if_exists()
                    .name("users_email_idx")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PurchaseBakery::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Purchase::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bakery::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Customers::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    ID,
    Name,
    Email,
    Photo,
    Verified,
    Password,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Bakery {
    Table,
    ID,
    Title,
    Image,
    Details,
    InStocks,
    Price,
    CreatedAt,
    RestockAt,
}

#[derive(DeriveIden)]
enum Purchase {
    Table,
    ID,
    CustomerID,
    SumPrice,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PurchaseBakery {
    Table,
    ID,
    PurchaseID,
    BakeryID,
    Quantity,
}

#[derive(DeriveIden)]
enum Customers {
    Table,
    ID,
    Name,
    LastName,
    Email,
}
