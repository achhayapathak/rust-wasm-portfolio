// ─────────────────────────────────────────────────────────────
// data.rs — All portfolio data lives here.
// Direct port of content.config.ts into Rust structs & constants.
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StockColor {
    Cream,
    Yellow,
    Magenta,
    Teal,
    Cobalt,
    Lime,
    Violet,
    Orange,
}

impl StockColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            StockColor::Cream => "cream",
            StockColor::Yellow => "yellow",
            StockColor::Magenta => "magenta",
            StockColor::Teal => "teal",
            StockColor::Cobalt => "cobalt",
            StockColor::Lime => "lime",
            StockColor::Violet => "violet",
            StockColor::Orange => "orange",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntryStatus {
    InProgress,
    Completed,
    Posted,
}

impl EntryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntryStatus::InProgress => "In-Progress",
            EntryStatus::Completed => "Completed",
            EntryStatus::Posted => "Posted",
        }
    }
}

pub struct SiteConfig {
    pub name: &'static str,
    pub title: &'static str,
    pub location: &'static str,
    pub email: &'static str,
    pub description: &'static str,
    pub site_url: &'static str,
}

pub struct SocialLink {
    pub platform: &'static str,
    pub url: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub show_in_footer: bool,
}

pub struct ContactCard {
    pub title: &'static str,
    pub icon: &'static str,
    pub stock: StockColor,
    pub url: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

pub struct WorkEntry {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub status: EntryStatus,
    pub stock: StockColor,
    pub tags: &'static [&'static str],
    pub summary: &'static str,
    pub bullets: &'static [&'static str],
}

pub struct ProjectEntry {
    pub name: &'static str,
    pub tagline: &'static str,
    pub period: &'static str,
    pub status: EntryStatus,
    pub stock: StockColor,
    pub tags: &'static [&'static str],
    pub summary: &'static str,
    pub github: Option<&'static str>,
    pub website: Option<&'static str>,
}

pub struct CertEntry {
    pub name: &'static str,
    pub issuer: &'static str,
    pub date: &'static str,
    pub status: EntryStatus,
    pub stock: StockColor,
    pub credential_id: Option<&'static str>,
}

pub struct EduEntry {
    pub institution: &'static str,
    pub degree: &'static str,
    pub period: &'static str,
    pub status: EntryStatus,
    pub stock: StockColor,
    pub summary: Option<&'static str>,
}

pub struct VolunteerEntry {
    pub org: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub status: Option<EntryStatus>,
    pub stock: StockColor,
    pub summary: Option<&'static str>,
}

pub struct TickerEntry {
    pub date: &'static str,
    pub org: &'static str,
    pub detail: &'static str,
    pub status: &'static str,
}

pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
    pub stock: StockColor,
}

pub struct SectionMeta {
    pub form: &'static str,
    pub subtitle: &'static str,
    pub title: &'static str,
    pub info: &'static str,
    pub stock: StockColor,
}

// ── Site Config ──────────────────────────────────────────────

pub static SITE: SiteConfig = SiteConfig {
    name: "Achhaya Pathak",
    title: "Software Engineer",
    location: "Gurugram, India",
    email: "info@achhaya.com",
    description: "IIT Guwahati MS alumnus and Software / Backend Engineer with 3+ years of experience architecting distributed systems, cloud infrastructure (AWS/GCP), high-performance web applications (Rust/WASM, Next.js), and autonomous AI agent pipelines.",
    site_url: "https://achhaya.com",
};

pub static SOCIAL: &[SocialLink] = &[
    SocialLink {
        platform: "LinkedIn",
        url: "https://linkedin.com/in/achhayapathak",
        label: "linkedin/achhayapathak",
        icon: "card",
        show_in_footer: true,
    },
    SocialLink {
        platform: "GitHub",
        url: "https://github.com/achhayapathak",
        label: "github/achhayapathak",
        icon: "code",
        show_in_footer: false,
    },
];

pub static CONTACT_CARDS: &[ContactCard] = &[
    ContactCard {
        title: "Email",
        icon: "mail",
        stock: StockColor::Lime,
        url: "https://mail.google.com/mail/?view=cm&fs=1&to=info@achhaya.com",
        label: "info@achhaya.com",
        description: "best for anything that needs a written trail",
    },
    ContactCard {
        title: "LinkedIn",
        icon: "card",
        stock: StockColor::Cobalt,
        url: "https://linkedin.com/in/achhayapathak",
        label: "linkedin/achhayapathak",
        description: "for the conversations that want a recruiter in them",
    },
    ContactCard {
        title: "Resume / CV",
        icon: "card",
        stock: StockColor::Magenta,
        url: "/Achhaya_Pathak.pdf",
        label: "resume",
        description: "comprehensive curriculum vitae covering experience, publications, and skills",
    },
    ContactCard {
        title: "GitHub",
        icon: "code",
        stock: StockColor::Cream,
        url: "https://github.com/achhayapathak",
        label: "github/achhayapathak",
        description: "the commit log is the CV that cannot round up",
    },
    ContactCard {
        title: "X (Twitter)",
        icon: "link",
        stock: StockColor::Teal,
        url: "https://x.com/frozen_parantha",
        label: "x/frozen_parantha",
        description: "for the conversations that want a bit of sass",
    },
    ContactCard {
        title: "LeetCode",
        icon: "code",
        stock: StockColor::Yellow,
        url: "https://leetcode.com/u/achhayapathak/",
        label: "leetcode/achhayapathak",
        description: "for problem solving, data structures, and algorithms",
    },
];

// ── Navigation ───────────────────────────────────────────────

pub static NAV_ITEMS: &[NavItem] = &[
    NavItem { label: "Overview", href: "/", stock: StockColor::Cream },
    NavItem { label: "Work", href: "/work", stock: StockColor::Lime },
    NavItem { label: "Projects", href: "/projects", stock: StockColor::Cobalt },
    NavItem { label: "Education", href: "/education", stock: StockColor::Yellow },
    NavItem { label: "Volunteering", href: "/volunteering", stock: StockColor::Magenta },
    NavItem { label: "Connect", href: "/contact", stock: StockColor::Orange },
];

// ── Work Experience ──────────────────────────────────────────

pub static WORK: &[WorkEntry] = &[
    WorkEntry {
        company: "JoinUp",
        role: "Co-Founder",
        period: "March 2025 - Present",
        status: EntryStatus::InProgress,
        stock: StockColor::Teal,
        tags: &["Leadership", "Growth", "Next.js", "TypeScript", "Python", "PostgreSQL", "Redis", "Docker", "AWS"],
        summary: "JoinUp is on a mission to make professional events more discoverable and accessible. By connecting attendees with relevant conferences, meetups, workshops, and networking opportunities, JoinUp helps people learn, connect, and grow their careers. For hosts, it\u{2019}s an entire operating system for them.",
        bullets: &[
            "Architected and scaled JoinUp (0\u{2192}1), a production-grade professional events marketplace, owning end-to-end backend, infrastructure, and deployment across AWS and GCP.",
            "Built an SEO-optimised Next.js (App Router) frontend with SSR, dynamic metadata, and sitemap/robots configuration, driving 3,000+ organic users/month, improving website conversion by 8%.",
            "Designed a geo-spatial discovery system (Google Maps API + PostgreSQL) delivering low-latency, location-based recommendations to 500+ DAU at sub-500ms p99.",
            "Implemented Python-based event crawlers and ETL pipelines with Redis caching, PostgreSQL-powered event matching, and built-in admin dashboard for event managers.",
        ],
    },
    WorkEntry {
        company: "Marlin",
        role: "Software Engineer",
        period: "Apr 2025 - May 2026",
        status: EntryStatus::Completed,
        stock: StockColor::Yellow,
        tags: &["Rust", "Agentic AI", "LLM", "RabbitMQ", "Kubernetes", "Microservices"],
        summary: "Built an autonomous LLM-driven trading AI agent that executes fully automated token trades on Hyperliquid, based on the condition set by user such as market trends, community sentiment, recent news, tweets, etc. It can also be used to copy trade of sharks or whales.",
        bullets: &[
            "Architected a fault-tolerant microservices system (Executor/Evaluator/Flusher) using RabbitMQ, enabling horizontal scalability, fault isolation, and zero-downtime processing 50+ real-time market signals per minute.",
            "Designed resilient queue-based orchestration with ack/nack semantics, retries, and auto-reconnect mechanisms, reducing system failures in AI-driven decision pipelines.",
            "Deployed and managed Kubernetes clusters within Confidential Virtual Machines (CVMs), establishing a zero-trust environment for highly secure AI-sensitive data transfer and processing.",
        ],
    },
    WorkEntry {
        company: "Gist Impact",
        role: "Software Development Engineer",
        period: "Feb 2024 - Apr 2025",
        status: EntryStatus::Completed,
        stock: StockColor::Cream,
        tags: &["Python", "PostgreSQL", "Snowflake", "ETL", "CI/CD", "Grafana Stack", "AWS"],
        summary: "Managed and scaled infrastructure for SaaS and DaaS platforms powering AI-assisted sustainability analytics workflows, processing ~5 TB of data per month. Worked on enhancing the existing platform\u{2019}s reliability, scalability, and performance.",
        bullets: &[
            "Re-engineered and optimised CI/CD pipelines using GitHub Actions, parallelising build stages, caching dependencies, and eliminating redundant test runs, cutting deployment time from 30 minutes to 5 minutes (6x improvement).",
            "Automated ETL pipelines in Snowflake, reducing data processing time by 80% and significantly improving the reliability and consistency of production data flows.",
            "Consolidated service logs into a centralised Grafana Stack observability framework, reducing error resolution time by 35% and increasing deployment velocity by 20%.",
            "Streamlined and optimised data ingestion and processing workflows for both SaaS and DaaS platforms, resulting in an 80% reduction in data processing time and a significant improvement in overall system reliability and performance.",
        ],
    },
    WorkEntry {
        company: "Aristocrat Gaming",
        role: "Game Developer",
        period: "Aug 2023 - Jan 2024",
        status: EntryStatus::Completed,
        stock: StockColor::Magenta,
        tags: &["JavaScript", "C++", "Simulation"],
        summary: "Designed mathematical models and wrote simulation codes for slot games, calculating return to player percentages, hit frequencies, and other metrics as appropriate for markets across the Globe. Crafted a highly optimised simulation code for the Super Grand Star slot game, establishing a benchmark of excellence within the organisation. The optimised code significantly reduced simulation runtime from 10 hours to 2.5 hours.",
        bullets: &[],
    },
    WorkEntry {
        company: "Indian Institute of Technology, Indore",
        role: "Summer Research Intern",
        period: "April 2021 - May 2021",
        status: EntryStatus::Completed,
        stock: StockColor::Violet,
        tags: &["MS Excel", "Python", "Tableau"],
        summary: "Conducted a seismic data analysis for the Hindukush region of 59255 earthquakes over the years 2000-2020. Reckoned the b-value for the target region as 1.136 \u{00B1} 0.007 using the Gutenberg\u{2013}Richter relation. Determined which locations are suitable for construction-related operations after carrying out a study based on a non-uniform division of the region.",
        bullets: &[],
    },
];

// ── Projects ─────────────────────────────────────────────────

pub static PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "JoinUp",
        tagline: "Event discovery & social platform",
        period: "2025 - Present",
        status: EntryStatus::InProgress,
        stock: StockColor::Lime,
        tags: &["Next.js", "TypeScript", "Python", "PostgreSQL", "Redis", "Docker", "AWS"],
        summary: "A full-stack event marketplace for discovering, creating, and joining local events. Built with a microservices backend in TypeScript and a Next.js frontend. JoinUp provides a personalised feed of events to users based on their interests and preferences. For hosts, its an end-to-end booking management and verification solution.",
        github: Some("https://github.com/joinupdev"),
        website: Some("https://joinup.dev"),
    },
    ProjectEntry {
        name: "Distributed Document Search",
        tagline: "Production grade distributed search service",
        period: "2026",
        status: EntryStatus::Completed,
        stock: StockColor::Magenta,
        tags: &["TypeScript", "Distributed Systems", "Multi-tenancy", "Elasticsearch", "Kafka", "Redis", "AWS"],
        summary: "A working prototype of a distributed document search service demonstrating enterprise-grade architectural patterns including multi-tenancy, async processing, caching, and full-text search. The service is capable of searching a text among millions of files with a sub-200 ms speed.",
        github: Some("https://github.com/achhayapathak/distributed-document-search"),
        website: None,
    },
    ProjectEntry {
        name: "Event Aggregation Engine",
        tagline: "Event aggregation engine for event discovery platforms",
        period: "2026 - Present",
        status: EntryStatus::InProgress,
        stock: StockColor::Teal,
        tags: &["Python", "Agentic AI", "RabbitMQ", "Firecrawl", "Redis", "VectorDB"],
        summary: "An event aggregation engine for event discovery platforms. It aggregates events from various sources and returns them in a centralised location with a consistent format and structure using Agentic AI.",
        github: Some("https://github.com/joinupdev"),
        website: None,
    },
    ProjectEntry {
        name: "Job Assist AI",
        tagline: "AI tool for job applications",
        period: "2026 - Present",
        status: EntryStatus::InProgress,
        stock: StockColor::Violet,
        tags: &["TypeScript", "Agentic AI", "Automation", "LLM", "Next.js"],
        summary: "An AI tool to help with job applications. It takes your resume, crawls the internet for relevant jobs for you, extracts the most important details from job descriptions along with recruiter details, and helps you generate an ATS friendly version of your resume for each application.",
        github: Some("https://github.com/achhayapathak/job-assist-ai-backend"),
        website: None,
    },
    ProjectEntry {
        name: "Termtalk",
        tagline: "Secure CLI Chat",
        period: "2024 - 2025",
        status: EntryStatus::Completed,
        stock: StockColor::Orange,
        tags: &["Websockets", "JavaScript", "CLI", "NPM"],
        summary: "Engineered an open-source NPM package enabling secure, real-time communication between remote systems via the terminal using Node.js and Socket.io. Maintained high-level security through measures like local server hosting, encrypted message transmission, and automatic chat disposal upon server termination, ensuring utmost security.",
        github: Some("https://github.com/achhayapathak/termtalk"),
        website: Some("https://www.npmjs.com/package/termtalk"),
    },
    ProjectEntry {
        name: "Research Publication - Blockchain",
        tagline: "Optimal Payment Splitting in Bitcoin\u{2019}s Lightning Network",
        period: "2023",
        status: EntryStatus::Completed,
        stock: StockColor::Cream,
        tags: &["Bitcoin", "Lightning Network", "Smart Contracts", "Blockchain"],
        summary: "Studied the Blockchain trilemma and its resolution through the Lightning Network\u{2019}s scalability solutions. Designed an efficient fee structure for the Lightning Network to prevent network saturation and ensure network liquidity. Developed a Dynamic Programming algorithm to minimize user expenditure on fees during payment transactions.",
        github: None,
        website: Some("https://drive.google.com/file/d/1u62MY5as4VtPEIxrFUJDhAmase0_LATJ/view?pli=1"),
    },
];

// ── Certifications ───────────────────────────────────────────

pub static CERTIFICATIONS: &[CertEntry] = &[
    CertEntry {
        name: "Summer Analytics Bootcamp",
        issuer: "INSPIRE Scholar",
        date: "2023",
        status: EntryStatus::Completed,
        stock: StockColor::Violet,
        credential_id: None,
    },
];

// ── Education ────────────────────────────────────────────────

pub static EDUCATION: &[EduEntry] = &[
    EduEntry {
        institution: "Indian Institute of Technology, Guwahati",
        degree: "Master\u{2019}s degree, Mathematics and Computing",
        period: "July 2021 - June 2023",
        status: EntryStatus::Completed,
        stock: StockColor::Violet,
        summary: Some("Completed my masters in mathematics and computing with specialization in courses such as Graph Theory, Optimization Theory, Probabilistic Method, Data Structure & Algorithm, Advanced Algorithms, etc. Also worked on research on Blockchain and wrote a research paper on it."),
    },
    EduEntry {
        institution: "Hansraj College, University of Delhi",
        degree: "Bachelor\u{2019}s degree, Mathematics",
        period: "July 2018 - June 2021",
        status: EntryStatus::Completed,
        stock: StockColor::Orange,
        summary: Some("Completed my bachelors in mathematics with minor in Computer Science. Key courses include Real Analysis, Number Theory, Data Structures and Algorithms, Operating Systems, Computer Networking, Probability Theory and Statistics, etc."),
    },
];

// ── Volunteering ─────────────────────────────────────────────

pub static VOLUNTEERING: &[VolunteerEntry] = &[
    VolunteerEntry {
        org: "Open Source Community",
        role: "Contributor",
        period: "2023 - Present",
        stock: StockColor::Cobalt,
        status: None,
        summary: Some("Contributing to open-source projects along with maintaining some of them. Reviewing Pull requests, responding to issues, etc."),
    },
    VolunteerEntry {
        org: "Guidance & Mentorship Programs",
        role: "Mentor",
        period: "2025 - Present",
        stock: StockColor::Teal,
        status: None,
        summary: Some("Guiding students in technical and career development. Mentoring fellow builders with technical aspect of their products."),
    },
    VolunteerEntry {
        org: "Technical Events",
        role: "Speaker",
        period: "2025 - Present",
        stock: StockColor::Violet,
        status: None,
        summary: Some("Spoken at various tech workshops and meetups on topics such as System Design, Agentic AI, Distributed Systems, etc."),
    },
];

// ── Ticker Entries ───────────────────────────────────────────

pub static TICKER: &[TickerEntry] = &[
    TickerEntry { date: "2026-02", org: "Job Assist AI", detail: "agentic ai \u{00B7} resume & job automation \u{00B7} next.js", status: "In-Progress" },
    TickerEntry { date: "2026-01", org: "Distributed Search", detail: "multi-tenant search \u{00B7} elasticsearch \u{00B7} kafka \u{00B7} redis", status: "Completed" },
    TickerEntry { date: "2025-03", org: "JoinUp", detail: "co-founder \u{00B7} events marketplace \u{00B7} aws & next.js", status: "In-Progress" },
    TickerEntry { date: "2025-04", org: "Marlin", detail: "software engineer \u{00B7} autonomous ai agent \u{00B7} rust & rabbitmq", status: "Completed" },
    TickerEntry { date: "2024-02", org: "Gist Impact", detail: "sde \u{00B7} etl pipelines \u{00B7} snowflake \u{00B7} grafana stack", status: "Completed" },
    TickerEntry { date: "2024-01", org: "Termtalk", detail: "secure cli chat \u{00B7} npm package \u{00B7} websockets", status: "Completed" },
    TickerEntry { date: "2023-08", org: "Aristocrat Gaming", detail: "game developer \u{00B7} simulation models \u{00B7} c++", status: "Completed" },
    TickerEntry { date: "2023-06", org: "IIT Guwahati", detail: "m.s. mathematics & computing \u{00B7} lightning network research", status: "Completed" },
];

// ── Section Form Labels (ledger-style) ───────────────────────

pub static SECTION_OVERVIEW: SectionMeta = SectionMeta {
    form: "OV-00",
    subtitle: "index of records",
    title: "Overview",
    info: "Everything on this page in brief \u{2014} the latest two entries from each section, with the full ledger one click away!",
    stock: StockColor::Cream,
};

pub static SECTION_WORK: SectionMeta = SectionMeta {
    form: "WK-01",
    subtitle: "roles & employment",
    title: "Work",
    info: "Every role I\u{2019}ve held, from big tech to early-stage startups \u{2014} detailed accounts of responsibilities, projects, and outcomes.",
    stock: StockColor::Lime,
};

pub static SECTION_PROJECTS: SectionMeta = SectionMeta {
    form: "PR-02",
    subtitle: "things i have built",
    title: "Projects",
    info: "Every product that I\u{2019}ve built, from tech demos to production systems \u{2014} deep dives into problem, tech stack, and results.",
    stock: StockColor::Cobalt,
};

pub static SECTION_CERTIFICATIONS: SectionMeta = SectionMeta {
    form: "CT-03",
    subtitle: "credentials & qualifications",
    title: "Certifications",
    info: "Professional certifications and technical credentials demonstrating verified expertise.",
    stock: StockColor::Violet,
};

pub static SECTION_EDUCATION: SectionMeta = SectionMeta {
    form: "ED-04",
    subtitle: "academic record",
    title: "Education",
    info: "Formal academic background, degrees, and foundational studies in mathematics and computer science.",
    stock: StockColor::Yellow,
};

pub static SECTION_VOLUNTEERING: SectionMeta = SectionMeta {
    form: "VL-05",
    subtitle: "community & leadership",
    title: "Volunteering",
    info: "Contributions to the open-source community, mentorship, and technical leadership outside of work.",
    stock: StockColor::Magenta,
};

pub static SECTION_CONTACT: SectionMeta = SectionMeta {
    form: "CX-06",
    subtitle: "get in touch",
    title: "Connect",
    info: "Direct lines, profiles worth your time. Open to backend and platform roles.",
    stock: StockColor::Orange,
};

pub static KNOWS_ABOUT: &[&str] = &[
    "Go", "Python", "TypeScript", "Kubernetes", "Docker", "Terraform",
    "AWS", "PostgreSQL", "Redis", "Kafka", "gRPC", "Distributed Systems",
];
