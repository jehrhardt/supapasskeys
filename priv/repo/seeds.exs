# Script for populating the database. You can run it as:
#
#     mix run priv/repo/seeds.exs
#
# Inside the script, you can read and write to any of your
# repositories directly:
#
#     Supapasskeys.Repo.insert!(%Supapasskeys.SomeSchema{})
#
# We recommend using the bang functions (`insert!`, `update!`
# and so on) as they will fail if something goes wrong.
{:ok, project} =
  Supapasskeys.Repo.insert(%Supapasskeys.Supabase.Project{
    name: "Supapasskeys",
    reference_id: "supapasskeys",
    database_url: "postgres://supapasskeys:supapasskeys@localhost:54329/postgres"
  })

Supapasskeys.Supabase.migrate_database(project)
