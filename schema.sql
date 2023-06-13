
CREATE TABLE `Announcement` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(256) NOT NULL,
  `content` text NOT NULL,
  `session_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `Announcement_FK` (`session_id`),
  CONSTRAINT `Announcement_FK` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `Choice`
--

CREATE TABLE `Choice` (
  `applicant_id` int(11) NOT NULL,
  `session_id` int(11) NOT NULL,
  `professor_id` int(11) NOT NULL,
  `order_of_priority` int(11) NOT NULL,
  `result` tinyint(1) DEFAULT NULL,
  PRIMARY KEY (`applicant_id`,`session_id`,`professor_id`),
  UNIQUE KEY `Choice_UN` (`applicant_id`,`order_of_priority`,`session_id`),
  KEY `Choice_FK_1` (`session_id`),
  KEY `Choice_FK` (`professor_id`,`session_id`),
  CONSTRAINT `Choice_FK` FOREIGN KEY (`professor_id`, `session_id`) REFERENCES `Theme` (`professor_id`, `session_id`) ON DELETE CASCADE,
  CONSTRAINT `Choice_FK_1` FOREIGN KEY (`applicant_id`) REFERENCES `User` (`id`) ON DELETE CASCADE
); 

--
-- Table structure for table `Module`
--

CREATE TABLE `Module` (
  `code` varchar(256) NOT NULL,
  `session_id` int(11) NOT NULL,
  PRIMARY KEY (`code`,`session_id`),
  KEY `Module_FK` (`session_id`),
  CONSTRAINT `Module_FK` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `Reclamation`
--

CREATE TABLE `Reclamation` (
  `applicant_id` int(11) NOT NULL,
  `module_id` varchar(256) NOT NULL,
  `session_id` int(11) NOT NULL,
  `content` text NOT NULL,
  PRIMARY KEY (`applicant_id`,`module_id`,`session_id`),
  CONSTRAINT `Reclamation_FK` FOREIGN KEY (`applicant_id`, `module_id`, `session_id`) REFERENCES `Result` (`applicant_id`, `module_id`, `session_id`) ON DELETE CASCADE
);

--
-- Table structure for table `Result`
--

CREATE TABLE `Result` (
  `applicant_id` int(11) NOT NULL,
  `module_id` varchar(256) NOT NULL,
  `session_id` int(11) NOT NULL,
  `corrector_1_id` int(11) DEFAULT NULL,
  `corrector_2_id` int(11) DEFAULT NULL,
  `corrector_3_id` int(11) DEFAULT NULL,
  `note_1` int(11) DEFAULT NULL,
  `note_2` int(11) DEFAULT NULL,
  `note_3` int(11) DEFAULT NULL,
  `note_final` int(11) DEFAULT NULL,
  `display_to_3rd_corrector` tinyint(1) NOT NULL DEFAULT 0,
  `display_to_applicant` tinyint(1) NOT NULL DEFAULT 0,
  `display_to_cfd` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`applicant_id`,`module_id`,`session_id`),
  KEY `Result_FK` (`module_id`,`session_id`),
  KEY `Result_FK_2` (`corrector_1_id`),
  KEY `Result_FK_3` (`corrector_2_id`),
  KEY `Result_FK_4` (`corrector_3_id`),
  KEY `Result_FK_1` (`applicant_id`,`session_id`),
  CONSTRAINT `Result_FK` FOREIGN KEY (`module_id`, `session_id`) REFERENCES `Module` (`code`, `session_id`) ON DELETE CASCADE,
  CONSTRAINT `Result_FK_1` FOREIGN KEY (`applicant_id`, `session_id`) REFERENCES `applicant_affectation` (`applicant_id`, `session_id`) ON DELETE CASCADE,
  CONSTRAINT `Result_FK_2` FOREIGN KEY (`corrector_1_id`) REFERENCES `User` (`id`) ON DELETE SET NULL,
  CONSTRAINT `Result_FK_3` FOREIGN KEY (`corrector_2_id`) REFERENCES `User` (`id`) ON DELETE SET NULL,
  CONSTRAINT `Result_FK_4` FOREIGN KEY (`corrector_3_id`) REFERENCES `User` (`id`) ON DELETE SET NULL
);

--
-- Table structure for table `Session`
--

CREATE TABLE `Session` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `virtual_platform_id` int(11) NOT NULL,
  `cfd_id` int(11) NOT NULL,
  `starting_time` bigint(20) NOT NULL,
  `ending_time` bigint(20) NOT NULL,
  `room_number` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `Session_FK` (`virtual_platform_id`),
  KEY `Session_FK_1` (`cfd_id`),
  CONSTRAINT `Session_FK` FOREIGN KEY (`virtual_platform_id`) REFERENCES `VirtualPlatform` (`vd_id`) ON DELETE CASCADE,
  CONSTRAINT `Session_FK_1` FOREIGN KEY (`cfd_id`) REFERENCES `User` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `Theme`
--

CREATE TABLE `Theme` (
  `session_id` int(11) NOT NULL,
  `professor_id` int(11) NOT NULL,
  `title` varchar(256) NOT NULL,
  `content` text NOT NULL,
  PRIMARY KEY (`professor_id`,`session_id`),
  KEY `Theme_FK_1` (`session_id`),
  CONSTRAINT `Theme_FK` FOREIGN KEY (`professor_id`) REFERENCES `User` (`id`) ON DELETE CASCADE,
  CONSTRAINT `Theme_FK_1` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `User`
--

CREATE TABLE `User` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(256) NOT NULL,
  `password` varchar(256) NOT NULL,
  `role` varchar(16) NOT NULL,
  `domaine` varchar(256) DEFAULT NULL,
  `specialty` varchar(256) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `User_UN` (`email`)
);

--
-- Table structure for table `VirtualPlatform`
--

CREATE TABLE `VirtualPlatform` (
  `vd_id` int(11) NOT NULL,
  `name` varchar(256) NOT NULL,
  PRIMARY KEY (`vd_id`),
  CONSTRAINT `VirtualPatform_FK` FOREIGN KEY (`vd_id`) REFERENCES `User` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `applicant_affectation`
--

CREATE TABLE `applicant_affectation` (
  `applicant_id` int(11) NOT NULL,
  `session_id` int(11) NOT NULL,
  `encoding` varchar(256) DEFAULT NULL,
  `presence` tinyint(1) DEFAULT NULL,
  PRIMARY KEY (`applicant_id`,`session_id`),
  UNIQUE KEY `applicant_affectation_UN` (`encoding`),
  KEY `applicant_affectation_FK_1` (`session_id`),
  CONSTRAINT `applicant_affectation_FK` FOREIGN KEY (`applicant_id`) REFERENCES `User` (`id`) ON DELETE CASCADE,
  CONSTRAINT `applicant_affectation_FK_1` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
);

--
-- Table structure for table `monitor_affectation`
--

CREATE TABLE `monitor_affectation` (
  `session_id` int(11) NOT NULL,
  `professor_id` int(11) NOT NULL,
  PRIMARY KEY (`session_id`,`professor_id`),
  KEY `monitor_affectation_FK` (`professor_id`),
  CONSTRAINT `monitor_affectation_FK` FOREIGN KEY (`professor_id`) REFERENCES `User` (`id`) ON DELETE CASCADE,
  CONSTRAINT `monitor_affectation_FK_1` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
);

