import "reflect-metadata";
import { DataSource } from "typeorm";
import { User } from "./entity/User";
import { Sample } from "./entity/Sample";

export const AppDataSource = new DataSource({
    type: "sqlite",
    database: "database.sqlite",
    synchronize: true,
    logging: false,
    entities: [User, Sample],
    migrations: [],
    subscribers: [],
});
