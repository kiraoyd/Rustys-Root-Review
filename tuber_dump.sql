--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2
-- Dumped by pg_dump version 15.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: iphistory; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.iphistory (
    id integer NOT NULL,
    updated_at timestamp(0) with time zone NOT NULL,
    deleted_at timestamp(0) with time zone,
    ip character varying(255) NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp(0) with time zone NOT NULL
);


ALTER TABLE public.iphistory OWNER TO tuber;

--
-- Name: iphistory_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.iphistory_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.iphistory_id_seq OWNER TO tuber;

--
-- Name: iphistory_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.iphistory_id_seq OWNED BY public.iphistory.id;


--
-- Name: mikro_orm_migrations; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.mikro_orm_migrations (
    id integer NOT NULL,
    name character varying(255),
    executed_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.mikro_orm_migrations OWNER TO tuber;

--
-- Name: mikro_orm_migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.mikro_orm_migrations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mikro_orm_migrations_id_seq OWNER TO tuber;

--
-- Name: mikro_orm_migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.mikro_orm_migrations_id_seq OWNED BY public.mikro_orm_migrations.id;


--
-- Name: profile; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.profile (
    id integer NOT NULL,
    updated_at timestamp(0) with time zone NOT NULL,
    deleted_at timestamp(0) with time zone,
    island_name character varying(255) NOT NULL,
    picture character varying(255) NOT NULL,
    turnips_held integer NOT NULL,
    price_paid integer NOT NULL,
    owner_id integer NOT NULL,
    created_at timestamp(0) with time zone NOT NULL
);


ALTER TABLE public.profile OWNER TO tuber;

--
-- Name: profile_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.profile_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.profile_id_seq OWNER TO tuber;

--
-- Name: profile_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.profile_id_seq OWNED BY public.profile.id;


--
-- Name: selling_price_history; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.selling_price_history (
    id integer NOT NULL,
    deleted_at timestamp(0) with time zone,
    island_id integer NOT NULL,
    date character varying(255) NOT NULL,
    price_am integer NOT NULL,
    price_pm integer NOT NULL,
    created_at timestamp(0) with time zone NOT NULL,
    updated_at timestamp(0) with time zone NOT NULL
);


ALTER TABLE public.selling_price_history OWNER TO tuber;

--
-- Name: selling_price_history_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.selling_price_history_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.selling_price_history_id_seq OWNER TO tuber;

--
-- Name: selling_price_history_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.selling_price_history_id_seq OWNED BY public.selling_price_history.id;


--
-- Name: transactions; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.transactions (
    id integer NOT NULL,
    created_at timestamp(0) with time zone NOT NULL,
    updated_at timestamp(0) with time zone NOT NULL,
    deleted_at timestamp(0) with time zone,
    number_sold integer NOT NULL,
    price_sold integer NOT NULL,
    profits integer NOT NULL,
    seller_id integer NOT NULL,
    host_id integer NOT NULL
);


ALTER TABLE public.transactions OWNER TO tuber;

--
-- Name: transactions_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.transactions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.transactions_id_seq OWNER TO tuber;

--
-- Name: transactions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.transactions_id_seq OWNED BY public.transactions.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: tuber
--

CREATE TABLE public.users (
    id integer NOT NULL,
    deleted_at timestamp(0) with time zone,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    role text NOT NULL,
    created_at timestamp(0) with time zone NOT NULL,
    updated_at timestamp(0) with time zone NOT NULL,
    CONSTRAINT users_role_check CHECK ((role = ANY (ARRAY['Admin'::text, 'User'::text])))
);


ALTER TABLE public.users OWNER TO tuber;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: tuber
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO tuber;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tuber
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: iphistory id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.iphistory ALTER COLUMN id SET DEFAULT nextval('public.iphistory_id_seq'::regclass);


--
-- Name: mikro_orm_migrations id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.mikro_orm_migrations ALTER COLUMN id SET DEFAULT nextval('public.mikro_orm_migrations_id_seq'::regclass);


--
-- Name: profile id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.profile ALTER COLUMN id SET DEFAULT nextval('public.profile_id_seq'::regclass);


--
-- Name: selling_price_history id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.selling_price_history ALTER COLUMN id SET DEFAULT nextval('public.selling_price_history_id_seq'::regclass);


--
-- Name: transactions id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.transactions ALTER COLUMN id SET DEFAULT nextval('public.transactions_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Data for Name: iphistory; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.iphistory (id, updated_at, deleted_at, ip, user_id, created_at) FROM stdin;
1	2023-05-23 05:21:09+00	\N	79.183.90.217	1	2023-05-23 05:21:09+00
2	2023-05-23 05:27:37+00	\N	154.244.237.108	5	2023-05-23 05:27:37+00
3	2023-05-23 05:28:08+00	\N	241.87.104.133	9	2023-05-23 05:28:08+00
4	2023-05-23 22:34:49+00	\N	3.101.181.14	13	2023-05-23 22:34:49+00
5	2023-05-23 22:34:49+00	\N	100.62.230.222	13	2023-05-23 22:34:49+00
6	2023-05-23 22:34:49+00	\N	176.41.122.41	13	2023-05-23 22:34:49+00
7	2023-05-23 22:34:49+00	\N	99.147.251.130	13	2023-05-23 22:34:49+00
8	2023-05-23 22:34:49+00	\N	147.241.205.159	13	2023-05-23 22:34:49+00
9	2023-05-23 22:34:49+00	\N	100.8.243.42	13	2023-05-23 22:34:49+00
10	2023-05-23 22:34:49+00	\N	165.163.177.220	13	2023-05-23 22:34:49+00
11	2023-05-23 22:34:49+00	\N	180.181.140.237	13	2023-05-23 22:34:49+00
12	2023-05-23 22:34:49+00	\N	115.223.38.144	13	2023-05-23 22:34:49+00
13	2023-05-23 22:34:49+00	\N	159.205.97.56	13	2023-05-23 22:34:49+00
\.


--
-- Data for Name: mikro_orm_migrations; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.mikro_orm_migrations (id, name, executed_at) FROM stdin;
\.


--
-- Data for Name: profile; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.profile (id, updated_at, deleted_at, island_name, picture, turnips_held, price_paid, owner_id, created_at) FROM stdin;
1	2023-05-23 05:21:09+00	\N	orjeene	http://placeholder.com/mypic.jpeg	1100	93	1	2023-05-23 05:21:09+00
2	2023-05-23 05:27:37+00	\N	orjeene	http://placeholder.com/mypic.jpeg	1100	93	5	2023-05-23 05:27:37+00
3	2023-05-23 05:28:08+00	\N	orjeene	http://placeholder.com/mypic.jpeg	1100	93	9	2023-05-23 05:28:08+00
4	2023-05-23 22:34:49+00	\N	orjeene	http://placeholder.com/mypic.jpeg	1100	93	13	2023-05-23 22:34:49+00
5	2023-05-23 22:34:49+00	\N	popcorn	http://placeholder.com/mypic.jpeg	100	100	13	2023-05-23 22:34:49+00
6	2023-05-23 22:34:49+00	\N	squirtle	http://placeholder.com/mypic.jpeg	2000	110	13	2023-05-23 22:34:49+00
7	2023-05-23 22:34:49+00	\N	pear	http://placeholder.com/mypic.jpeg	4050	106	13	2023-05-23 22:34:49+00
8	2023-05-23 22:34:49+00	\N	fakeorjeene	http://placeholder.com/mypic.jpeg	1100	93	13	2023-05-23 22:34:49+00
\.


--
-- Data for Name: selling_price_history; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.selling_price_history (id, deleted_at, island_id, date, price_am, price_pm, created_at, updated_at) FROM stdin;
1	\N	1	2023-05-22	1100	93	2023-05-23 05:21:09+00	2023-05-23 05:21:09+00
2	\N	2	2023-05-22	1100	93	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00
3	\N	3	2023-05-22	1100	93	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00
4	\N	4	2023-05-23	1100	93	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00
\.


--
-- Data for Name: transactions; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.transactions (id, created_at, updated_at, deleted_at, number_sold, price_sold, profits, seller_id, host_id) FROM stdin;
2	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00	\N	2000	110	50000	5	2
3	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00	\N	2000	110	50000	9	3
4	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00	\N	2000	110	50000	13	4
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: tuber
--

COPY public.users (id, deleted_at, name, email, password, role, created_at, updated_at) FROM stdin;
1	\N	kirak	email@email.com	password	Admin	2023-05-23 05:21:09+00	2023-05-23 05:21:09+00
2	\N	otherGuy	email2@email.com	password	User	2023-05-23 05:21:09+00	2023-05-23 05:21:09+00
3	\N	olimo	email3@email.com	password	User	2023-05-23 05:21:09+00	2023-05-23 05:21:09+00
4	\N	bender	email4@email.com	password	User	2023-05-23 05:21:09+00	2023-05-23 05:21:09+00
5	\N	kirak	email@email.com	password	Admin	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00
6	\N	otherGuy	email2@email.com	password	User	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00
7	\N	olimo	email3@email.com	password	User	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00
8	\N	bender	email4@email.com	password	User	2023-05-23 05:27:37+00	2023-05-23 05:27:37+00
9	\N	kirak	email@email.com	password	Admin	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00
10	\N	otherGuy	email2@email.com	password	User	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00
11	\N	olimo	email3@email.com	password	User	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00
12	\N	bender	email4@email.com	password	User	2023-05-23 05:28:08+00	2023-05-23 05:28:08+00
13	\N	kirak	email@email.com	password	Admin	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00
14	\N	otherGuy	email2@email.com	password	User	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00
15	\N	olimo	email3@email.com	password	User	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00
16	\N	bender	email4@email.com	password	User	2023-05-23 22:34:49+00	2023-05-23 22:34:49+00
\.


--
-- Name: iphistory_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.iphistory_id_seq', 13, true);


--
-- Name: mikro_orm_migrations_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.mikro_orm_migrations_id_seq', 1, false);


--
-- Name: profile_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.profile_id_seq', 8, true);


--
-- Name: selling_price_history_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.selling_price_history_id_seq', 4, true);


--
-- Name: transactions_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.transactions_id_seq', 4, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tuber
--

SELECT pg_catalog.setval('public.users_id_seq', 16, true);


--
-- Name: iphistory iphistory_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.iphistory
    ADD CONSTRAINT iphistory_pkey PRIMARY KEY (id);


--
-- Name: mikro_orm_migrations mikro_orm_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.mikro_orm_migrations
    ADD CONSTRAINT mikro_orm_migrations_pkey PRIMARY KEY (id);


--
-- Name: profile profile_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.profile
    ADD CONSTRAINT profile_pkey PRIMARY KEY (id);


--
-- Name: selling_price_history selling_price_history_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.selling_price_history
    ADD CONSTRAINT selling_price_history_pkey PRIMARY KEY (id);


--
-- Name: transactions transactions_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: iphistory iphistory_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.iphistory
    ADD CONSTRAINT iphistory_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id) ON UPDATE CASCADE;


--
-- Name: profile profile_owner_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.profile
    ADD CONSTRAINT profile_owner_id_foreign FOREIGN KEY (owner_id) REFERENCES public.users(id) ON UPDATE CASCADE;


--
-- Name: selling_price_history selling_price_history_island_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.selling_price_history
    ADD CONSTRAINT selling_price_history_island_id_foreign FOREIGN KEY (island_id) REFERENCES public.profile(id) ON UPDATE CASCADE;


--
-- Name: transactions transactions_host_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_host_id_foreign FOREIGN KEY (host_id) REFERENCES public.profile(id) ON UPDATE CASCADE;


--
-- Name: transactions transactions_seller_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: tuber
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_seller_id_foreign FOREIGN KEY (seller_id) REFERENCES public.users(id) ON UPDATE CASCADE;


--
-- PostgreSQL database dump complete
--

