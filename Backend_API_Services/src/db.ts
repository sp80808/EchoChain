
import sqlite3 from 'sqlite3';
import { open } from 'sqlite';

const DB_PATH = './database.db';

export async function openDb() {
  return open({
    filename: DB_PATH,
    driver: sqlite3.Database
  });
}

export async function createSchema() {
  const db = await openDb();
  await db.exec(`
    CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      username TEXT NOT NULL UNIQUE,
      password TEXT NOT NULL
    );
  `);
  await db.close();
}

createSchema().catch(err => {
  console.error('Error creating database schema:', err);
});
