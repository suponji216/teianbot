/*
  Warnings:

  - A unique constraint covering the columns `[pairId]` on the table `Inout` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "Inout" ADD COLUMN     "pairId" INTEGER;

-- CreateIndex
CREATE UNIQUE INDEX "Inout_pairId_key" ON "Inout"("pairId");

-- AddForeignKey
ALTER TABLE "Inout" ADD CONSTRAINT "Inout_pairId_fkey" FOREIGN KEY ("pairId") REFERENCES "Inout"("id") ON DELETE SET NULL ON UPDATE CASCADE;
