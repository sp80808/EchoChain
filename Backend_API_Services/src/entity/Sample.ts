import { Entity, PrimaryColumn, Column } from "typeorm";

@Entity()
export class Sample {
    @PrimaryColumn()
    id!: string; // Using p2pContentId as the primary key

    @Column()
    title!: string;

    @Column()
    artist!: string;

    @Column()
    duration!: string;

    @Column()
    category!: string;

    @Column({ unique: true })
    p2pContentId!: string;

    @Column("double")
    price!: number;

    @Column()
    ownerAddress!: string;

    @Column()
    blockchainHash!: string;
}
