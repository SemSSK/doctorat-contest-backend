-- MariaDB dump 10.19  Distrib 10.6.11-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: Edl
-- ------------------------------------------------------
-- Server version	10.6.11-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `Announcement`
--

DROP TABLE IF EXISTS `Announcement`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Announcement` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(256) NOT NULL,
  `content` text NOT NULL,
  `session_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `Announcement_FK` (`session_id`),
  CONSTRAINT `Announcement_FK` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Announcement`
--

LOCK TABLES `Announcement` WRITE;
/*!40000 ALTER TABLE `Announcement` DISABLE KEYS */;
INSERT INTO `Announcement` VALUES (3,'Hello','zejfezfo',5),(4,'Hello','zejfezfo',5),(5,'Hello','zejfezfo',5);
/*!40000 ALTER TABLE `Announcement` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Choice`
--

DROP TABLE IF EXISTS `Choice`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Choice`
--

LOCK TABLES `Choice` WRITE;
/*!40000 ALTER TABLE `Choice` DISABLE KEYS */;
/*!40000 ALTER TABLE `Choice` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Module`
--

DROP TABLE IF EXISTS `Module`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Module` (
  `code` varchar(256) NOT NULL,
  `session_id` int(11) NOT NULL,
  PRIMARY KEY (`code`,`session_id`),
  KEY `Module_FK` (`session_id`),
  CONSTRAINT `Module_FK` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Module`
--

LOCK TABLES `Module` WRITE;
/*!40000 ALTER TABLE `Module` DISABLE KEYS */;
INSERT INTO `Module` VALUES ('Algo',5),('CRI',5),('TQL',5);
/*!40000 ALTER TABLE `Module` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Reclamation`
--

DROP TABLE IF EXISTS `Reclamation`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Reclamation` (
  `applicant_id` int(11) NOT NULL,
  `module_id` varchar(256) NOT NULL,
  `session_id` int(11) NOT NULL,
  `content` text NOT NULL,
  PRIMARY KEY (`applicant_id`,`module_id`,`session_id`),
  CONSTRAINT `Reclamation_FK` FOREIGN KEY (`applicant_id`, `module_id`, `session_id`) REFERENCES `Result` (`applicant_id`, `module_id`, `session_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Reclamation`
--

LOCK TABLES `Reclamation` WRITE;
/*!40000 ALTER TABLE `Reclamation` DISABLE KEYS */;
/*!40000 ALTER TABLE `Reclamation` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Result`
--

DROP TABLE IF EXISTS `Result`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Result`
--

LOCK TABLES `Result` WRITE;
/*!40000 ALTER TABLE `Result` DISABLE KEYS */;
INSERT INTO `Result` VALUES (25,'Algo',5,23,24,30,12,13,NULL,NULL,0,1,1),(26,'Algo',5,23,24,30,12,11,NULL,NULL,0,1,1),(27,'Algo',5,23,24,30,12,16,15,NULL,0,1,1);
/*!40000 ALTER TABLE `Result` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Session`
--

DROP TABLE IF EXISTS `Session`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
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
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Session`
--

LOCK TABLES `Session` WRITE;
/*!40000 ALTER TABLE `Session` DISABLE KEYS */;
INSERT INTO `Session` VALUES (5,21,22,0,10,1);
/*!40000 ALTER TABLE `Session` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Theme`
--

DROP TABLE IF EXISTS `Theme`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Theme` (
  `session_id` int(11) NOT NULL,
  `professor_id` int(11) NOT NULL,
  `title` varchar(256) NOT NULL,
  `content` text NOT NULL,
  PRIMARY KEY (`professor_id`,`session_id`),
  KEY `Theme_FK_1` (`session_id`),
  CONSTRAINT `Theme_FK` FOREIGN KEY (`professor_id`) REFERENCES `User` (`id`) ON DELETE CASCADE,
  CONSTRAINT `Theme_FK_1` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Theme`
--

LOCK TABLES `Theme` WRITE;
/*!40000 ALTER TABLE `Theme` DISABLE KEYS */;
/*!40000 ALTER TABLE `Theme` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `User`
--

DROP TABLE IF EXISTS `User`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `User` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(256) NOT NULL,
  `password` varchar(256) NOT NULL,
  `role` varchar(16) NOT NULL,
  `domaine` varchar(256) DEFAULT NULL,
  `specialty` varchar(256) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `User_UN` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=31 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `User`
--

LOCK TABLES `User` WRITE;
/*!40000 ALTER TABLE `User` DISABLE KEYS */;
INSERT INTO `User` VALUES (1,'admin@email.com','1234','Admin',NULL,NULL),(21,'A','hKurzxO','ViceDoyen','Informatique',NULL),(22,'B','IFde6el','CFD','Informatique','GL'),(23,'C','U8exkQE','Professor','Informatique','GL'),(24,'D','XCHp8Gz','Professor','Informatique','GL'),(25,'E','nkzboMJ','Applicant','Informatique','GL'),(26,'F','gIPZzI0','Applicant','Informatique','GL'),(27,'G','39Qf8YB','Applicant','Informatique','GL'),(28,'H','oYjlr74','Applicant','Informatique','GL'),(29,'I','mB3sCb1','Applicant','Informatique','GL'),(30,'J','1234','Professor','Informatique','GL');
/*!40000 ALTER TABLE `User` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `VirtualPlatform`
--

DROP TABLE IF EXISTS `VirtualPlatform`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `VirtualPlatform` (
  `vd_id` int(11) NOT NULL,
  `name` varchar(256) NOT NULL,
  PRIMARY KEY (`vd_id`),
  CONSTRAINT `VirtualPatform_FK` FOREIGN KEY (`vd_id`) REFERENCES `User` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `VirtualPlatform`
--

LOCK TABLES `VirtualPlatform` WRITE;
/*!40000 ALTER TABLE `VirtualPlatform` DISABLE KEYS */;
INSERT INTO `VirtualPlatform` VALUES (21,'Informatique');
/*!40000 ALTER TABLE `VirtualPlatform` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `applicant_affectation`
--

DROP TABLE IF EXISTS `applicant_affectation`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `applicant_affectation`
--

LOCK TABLES `applicant_affectation` WRITE;
/*!40000 ALTER TABLE `applicant_affectation` DISABLE KEYS */;
INSERT INTO `applicant_affectation` VALUES (25,5,'1',1),(26,5,'2',1),(27,5,'3',1),(28,5,NULL,0);
/*!40000 ALTER TABLE `applicant_affectation` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `monitor_affectation`
--

DROP TABLE IF EXISTS `monitor_affectation`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `monitor_affectation` (
  `session_id` int(11) NOT NULL,
  `professor_id` int(11) NOT NULL,
  PRIMARY KEY (`session_id`,`professor_id`),
  KEY `monitor_affectation_FK` (`professor_id`),
  CONSTRAINT `monitor_affectation_FK` FOREIGN KEY (`professor_id`) REFERENCES `User` (`id`) ON DELETE CASCADE,
  CONSTRAINT `monitor_affectation_FK_1` FOREIGN KEY (`session_id`) REFERENCES `Session` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `monitor_affectation`
--

LOCK TABLES `monitor_affectation` WRITE;
/*!40000 ALTER TABLE `monitor_affectation` DISABLE KEYS */;
INSERT INTO `monitor_affectation` VALUES (5,23);
/*!40000 ALTER TABLE `monitor_affectation` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-04-15 22:08:42
