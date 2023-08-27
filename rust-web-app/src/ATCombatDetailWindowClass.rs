// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATCombatDetailWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ATCombatDetailWindowClass : WindowClass
  {
     int okid;
     int tbacknr;
     int oktextid;
     int noteid;
     int note2id;
     int cloudid;
     int Pic1Id;
     int TAid;
     int Ta2Id;
     DateTime lasttime;
     int FromMessage;
     bool ForwardKey;
     bool LastDrawAfterEnd;
     int Hn;
     int OptionsListId;
     ATListClass OptionsListObj;
     int OptionsList1Id;
     ATListClass OptionsList1Obj;
     int OptionsList2Id;
     ATListClass OptionsList2Obj;
     int OptionsList3Id;
     ATListClass OptionsList3Obj;
     int OptionsList4Id;
     ATListClass OptionsList4Obj;
     int OptionsList5Id;
     ATListClass OptionsList5Obj;
     int OptionsList6Id;
     ATListClass OptionsList6Obj;
     int detailnr;
     int detailnr2;
     int detailnr3;
     int detailnr4;
     int detailnr1;

    pub ATCombatDetailWindowClass(ref tGame: GameClass, int thn)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.Hn = thn;
      this.dostuff();
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr1 = -1;
    }

    pub ATCombatDetailWindowClass(ref tGame: GameClass, int thn, bool OldGui)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.Hn = thn;
      this.dostuff();
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr1 = -1;
    }

    pub fn HandleToolTip(int x, int y)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff()
    {
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.TAid > 0)
      {
        this.RemoveSubPart(this.TAid);
        this.TAid = 0;
      }
      SizeF sizeF = SizeF::new();
      bool flag;
      if ( this.game.Data.RuleVar[839] >= 1.0)
      {
        flag = true;
        this.NewBackGroundAndClearAll(1024, 768, -1);
      }
      else
      {
        flag = false;
        this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
      }
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (flag)
        DrawMod.DrawMessFrame(ref this.OwnBitmap, ref Expression, 0, 0, 1024, 768);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      DrawMod.DrawTextVic2(ref Expression, "COMBAT ROUNDS", this.game.VicFont2, 10, 8, this.game.VicColor2, this.game.VicColor1Shade);
      this.OptionsListObj = ATListClass::new();
      if (this.detailnr > this.game.TempCombat.CombatRound)
        this.detailnr = -1;
      let mut tlistselect1: i32 =  -1;
      let mut num1: i32 =  -1;
      SubPartClass tsubpart;
      if (this.game.TempCombat.CombatRound > -1)
      {
        let mut combatRound: i32 =  this.game.TempCombat.CombatRound;
        for (let mut index: i32 =  1; index <= combatRound; index += 1)
        {
          num1 += 1;
          if (this.detailnr <= 0)
            this.detailnr = index;
          if (this.detailnr == index)
            tlistselect1 = num1;
          this.OptionsListObj.add("Round " + Strings.Trim(Conversion.Str( index)), index);
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsListObj, 13, 280, tlistselect1, this.game, true, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 25);
          this.OptionsListId = this.AddSubPart(ref tsubpart, 10, 25, 280, 224, 0);
        }
      }
      DrawMod.DrawTextVic2(ref Expression, "UNITS", this.game.VicFont2, 10, 252, this.game.VicColor2, this.game.VicColor1Shade);
      this.OptionsList1Obj = ATListClass::new();
      if (this.detailnr1 > this.game.TempCombat.UCounter)
        this.detailnr1 = -1;
      let mut tlistselect2: i32 =  -1;
      let mut num2: i32 =  -1;
      if (this.game.TempCombat.UCounter > -1)
      {
        let mut ucounter: i32 =  this.game.TempCombat.UCounter;
        for (let mut tdata: i32 =  0; tdata <= ucounter; tdata += 1)
        {
          num2 += 1;
          if (this.detailnr1 == -1)
            this.detailnr1 = tdata;
          if (this.detailnr1 == tdata)
            tlistselect2 = num2;
          this.OptionsList1Obj.add(Strings.Left(this.game.Data.UnitObj[this.game.TempCombat.UList[tdata].UNr].Name, 30), tdata, Strings.Left(this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.TempCombat.UList[tdata].UNr].Regime].Name, 20));
        }
        if (this.OptionsList1Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList1Id)].Refresh(this.OptionsList1Obj, tlistselect2);
          this.SubPartFlag[this.SubpartNr(this.OptionsList1Id)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsList1Obj, 14, 280, tlistselect2, this.game, true, tHeaderCenter: false, tShowPair: true, tValueWidth: 120, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 270);
          this.OptionsList1Id = this.AddSubPart(ref tsubpart, 10, 270, 280, 240, 0);
        }
      }
      DrawMod.DrawTextVic2(ref Expression, "UNIT REPORTS", this.game.VicFont2, 10, 517, this.game.VicColor2, this.game.VicColor1Shade);
      this.OptionsList2Obj = ATListClass::new();
      let mut tlistselect3: i32 =  -1;
      if (this.detailnr1 > -1)
      {
        let mut num3: i32 =  -1;
        let mut repCounter: i32 =  this.game.TempCombat.RepCounter;
        int num4;
        for (let mut tdata: i32 =  0; tdata <= repCounter; tdata += 1)
        {
          if (this.game.TempCombat.RepFrom[tdata] == this.detailnr1 & this.game.TempCombat.RepRound[tdata] == this.detailnr)
          {
            num3 += 1;
            if (num3 == 0)
              num4 = tdata;
            if (this.detailnr2 == tdata)
              tlistselect3 = num3;
            this.OptionsList2Obj.add(Strings.Left(this.game.TempCombat.RepTitle[tdata], 55), tdata);
          }
        }
        if (tlistselect3 == -1)
          this.detailnr2 = -1;
        if (tlistselect3 == -1)
          this.detailnr2 = -1;
        if (num3 > -1 & this.detailnr4 == -1 & tlistselect3 == -1)
        {
          tlistselect3 = 0;
          this.detailnr2 = num4;
        }
      }
      if (this.OptionsList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect3);
        this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
      }
      else
      {
        tsubpart =  new ATListSubPartClass(this.OptionsList2Obj, 11, 280, tlistselect3, this.game, true, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 535);
        this.OptionsList2Id = this.AddSubPart(ref tsubpart, 10, 535, 280, 192, 0);
      }
      DrawMod.DrawTextVic2(ref Expression, "INDIVIDUAL LIST", this.game.VicFont2, 310, 8, this.game.VicColor2, this.game.VicColor1Shade);
      this.OptionsList3Obj = ATListClass::new();
      let mut tlistselect4: i32 =  -1;
      if (this.detailnr1 > -1)
      {
        let mut num5: i32 =  -1;
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut tdata: i32 =  0; tdata <= icounter; tdata += 1)
        {
          if (this.game.TempCombat.IList[tdata].IUnr == this.game.TempCombat.UList[this.detailnr1].UNr)
          {
            num5 += 1;
            if (this.detailnr3 == -1)
              this.detailnr3 = tdata;
            if (this.detailnr3 == tdata)
              tlistselect4 = num5;
            tvalue: String;
            tvalue2: String;
            if (!Information.IsNothing( this.game.TempCombat.IList[tdata].IHistoricState))
            {
              tvalue = this.game.TempCombat.IList[tdata].IHistoricState[this.detailnr];
              tvalue2 = this.game.TempCombat.IList[tdata].IHistoricState2[this.detailnr];
            }
            else
            {
              tvalue = "";
              tvalue2 = "";
            }
            this.OptionsList3Obj.add(Strings.Left(Strings.Trim(Conversion.Str( this.game.TempCombat.IList[tdata].IID)) + ") " + this.game.Data.SFTypeObj[this.game.TempCombat.IList[tdata].ISFType].Name, 30), tdata, tvalue, tvalue2);
          }
        }
        if (tlistselect4 == -1)
          this.detailnr3 = -1;
      }
      if (this.OptionsList3Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect4);
        this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
      }
      else
      {
        tsubpart =  new ATListSubPartClass(this.OptionsList3Obj, 36, 280, tlistselect4, this.game, true, tHeaderCenter: false, tShowPair: true, tValueWidth: 140, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 310, bby: 25);
        this.OptionsList3Id = this.AddSubPart(ref tsubpart, 310, 25, 280, 592, 0);
      }
      DrawMod.DrawTextVic2(ref Expression, "INDIVIDUAL REPORTS", this.game.VicFont2, 610, 8, this.game.VicColor2, this.game.VicColor1Shade);
      this.OptionsList4Obj = ATListClass::new();
      let mut tlistselect5: i32 =  -1;
      if (this.detailnr3 > -1)
      {
        let mut num6: i32 =  -1;
        let mut repCounter: i32 =  this.game.TempCombat.RepCounter;
        int num7;
        for (let mut tdata: i32 =  0; tdata <= repCounter; tdata += 1)
        {
          if (this.game.TempCombat.RepFrom[tdata] == this.detailnr3 + 10000 & this.game.TempCombat.RepRound[tdata] == this.detailnr)
          {
            num6 += 1;
            if (num6 == 0)
              num7 = tdata;
            if (this.detailnr4 == tdata)
              tlistselect5 = num6;
            this.OptionsList4Obj.add(Strings.Left(this.game.TempCombat.RepTitle[tdata], 85), tdata);
          }
        }
        if (tlistselect5 == -1)
          this.detailnr4 = -1;
        if (this.detailnr2 == -1 & num6 > -1 & tlistselect5 == -1)
        {
          tlistselect5 = 0;
          this.detailnr4 = num7;
        }
      }
      if (this.OptionsList4Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect5);
        this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
      }
      else
      {
        tsubpart =  new ATListSubPartClass(this.OptionsList4Obj, 15, 380, tlistselect5, this.game, true, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 25);
        this.OptionsList4Id = this.AddSubPart(ref tsubpart, 610, 25, 380, 256, 0);
      }
      this.RemoveSubPart(this.TAid);
      this.RemoveSubPart(this.OptionsList5Id);
      this.OptionsList5Id = 0;
      this.RemoveSubPart(this.OptionsList6Id);
      this.OptionsList6Id = 0;
      int index1;
      if (this.detailnr2 > -1)
        index1 = this.detailnr2;
      if (this.detailnr4 > -1)
        index1 = this.detailnr4;
      if (index1 > -1)
      {
        str: String = Strings.UCase(this.game.TempCombat.RepTitle[index1]) + "\r\n" + "\r\n" + this.game.TempCombat.RepText[index1];
        if (Strings.InStr(str, "[tab]") == 0 & this.detailnr2 == index1)
          str = "[tab]Unit Report," + str + "[/tab]";
        if (Strings.InStr(str, "[tab]") == 0 & this.detailnr4 == index1)
          str = "[tab]Individual Report," + str + "[/tab]";
        if (this.game.TempCombat.RepType[index1] == 1)
        {
          if (flag)
          {
            tsubpart =  new TextAreaClass2(this.game, 380, 6, this.game.VicFont3, str, 15, ref this.OwnBitmap, 610, 300);
            this.TAid = this.AddSubPart(ref tsubpart, 610, 300, 380, 105, 0);
          }
          else
          {
            tsubpart =  new TextAreaClass(this.game, 380, 6, this.game.VicFont3, "", false, str, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 300);
            this.TAid = this.AddSubPart(ref tsubpart, 610, 300, 380, 105, 0);
          }
          DrawMod.DrawTextVic2(ref Expression, "ATTACK SCORE MODS", this.game.VicFont2, 610, 447, this.game.VicColor2, this.game.VicColor1Shade);
          this.OptionsList5Obj = ATListClass::new();
          let mut index2: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index2, 0, index1]) && this.game.TempCombat.RepCom[index2, 0, index1].Length > 0 & (index2 == 1 | Operators.CompareString(this.game.TempCombat.RepCom[index2 - 1, 2, index1], this.game.TempCombat.RepCom[index2, 2, index1], false) != 0 | Operators.CompareString(this.game.TempCombat.RepCom[index2, 0, index1], "After mods", false) == 0) && Operators.CompareString(this.game.TempCombat.RepCom[index2, 2, index1], "?", false) != 0)
              this.OptionsList5Obj.add(this.game.TempCombat.RepCom[index2, 0, index1], -1, this.game.TempCombat.RepCom[index2, 1, index1], Conversions.ToString(Math.Round( Conversions.ToSingle(this.game.TempCombat.RepCom[index2, 2, index1]), 1)));
            index2 += 1;
          }
          while (index2 <= 30);
          tsubpart =  new ATListSubPartClass(this.OptionsList5Obj, 15, 170, tlistselect5, this.game, true, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 80, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 465);
          this.OptionsList5Id = this.AddSubPart(ref tsubpart, 610, 465, 170, 256, 0);
          DrawMod.DrawTextVic2(ref Expression, "DEFENDER SCORE MODS", this.game.VicFont2, 810, 447, this.game.VicColor2, this.game.VicColor1Shade);
          this.OptionsList6Obj = ATListClass::new();
          let mut index3: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index3, 3, index1]) & !Information.IsNothing( this.game.TempCombat.RepCom[index3, 5, index1]) && Operators.CompareString(this.game.TempCombat.RepCom[index3, 5, index1], "?", false) != 0 & this.game.TempCombat.RepCom[index3, 3, index1].Length > 0 & (index3 == 1 | Operators.CompareString(this.game.TempCombat.RepCom[index3 - 1, 5, index1], this.game.TempCombat.RepCom[index3, 5, index1], false) != 0 | Operators.CompareString(this.game.TempCombat.RepCom[index3, 3, index1], "After mods", false) == 0) && Operators.CompareString(this.game.TempCombat.RepCom[index3, 5, index1], "?", false) != 0)
              this.OptionsList6Obj.add(this.game.TempCombat.RepCom[index3, 3, index1], -1, this.game.TempCombat.RepCom[index3, 4, index1], Conversions.ToString(Math.Round( Conversions.ToSingle(this.game.TempCombat.RepCom[index3, 5, index1]), 1)));
            index3 += 1;
          }
          while (index3 <= 30);
          tsubpart =  new ATListSubPartClass(this.OptionsList6Obj, 15, 170, tlistselect5, this.game, true, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 80, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 810, bby: 465);
          this.OptionsList6Id = this.AddSubPart(ref tsubpart, 810, 465, 170, 256, 0);
        }
        else if (this.game.TempCombat.RepType[index1] == 2)
        {
          DrawMod.DrawTextVic2(ref Expression, "STATUS", this.game.VicFont2, 610, 300, this.game.VicColor2, this.game.VicColor1Shade);
          this.OptionsList5Obj = ATListClass::new();
          let mut index4: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index4, 0, index1]) && this.game.TempCombat.RepCom[index4, 0, index1].Length > 0)
              this.OptionsList5Obj.add(this.game.TempCombat.RepCom[index4, 0, index1], -1, this.game.TempCombat.RepCom[index4, 1, index1]);
            index4 += 1;
          }
          while (index4 <= 30);
          tsubpart =  new ATListSubPartClass(this.OptionsList5Obj, 19, 380, tlistselect5, this.game, true, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 200, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 317);
          this.OptionsList5Id = this.AddSubPart(ref tsubpart, 610, 317, 380, 320, 0);
        }
        else if (flag)
        {
          tsubpart =  new TextAreaClass2(this.game, 380, 26, this.game.MarcFont5, str, 15, ref this.OwnBitmap, 610, 300);
          this.TAid = this.AddSubPart(ref tsubpart, 610, 300, 380, 405, 0);
        }
        else
        {
          tsubpart =  new TextAreaClass(this.game, 380, 26, this.game.gamefont1b, "", false, str, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 300);
          this.TAid = this.AddSubPart(ref tsubpart, 610, 300, 380, 405, 0);
        }
      }
      if (this.game.EditObj.CombatOneSentence.Length > 1)
      {
        if (flag)
        {
          tsubpart =  new TextAreaClass2(this.game, 290, 3, this.game.VicFont3, this.game.EditObj.CombatOneSentence, 15, ref this.OwnBitmap, 310, 620);
          this.Ta2Id = this.AddSubPart(ref tsubpart, 310, 620, 290, 60, 0);
        }
        else
        {
          tsubpart =  new TextAreaClass(this.game, 290, 3, this.game.VicFont3, "", false, this.game.EditObj.CombatOneSentence, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 310, bby: 620);
          this.Ta2Id = this.AddSubPart(ref tsubpart, 310, 620, 290, 60, 0);
        }
      }
      if (flag)
      {
        tsubpart =  new TextButtonPartClass("BACK", 100, "Click to return to regular combat screen\r\nOr press any key instead.", ref this.OwnBitmap, 435, this.Hn - 40, theight: 20, usefont: this.game.VicFont2, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart(ref tsubpart, 460, this.Hn - 40, 100, 20, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Back", 100, tBackbitmap: (ref this.OwnBitmap), bbx: 460, bby: (this.Hn - 70));
        this.okid = this.AddSubPart(ref tsubpart, 460, this.Hn - 70, 100, 35, 1);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub int GetSfNrBitmap(int sfnr, int att)
    {
      let mut type: i32 =  this.game.Data.SFObj[sfnr].Type;
      let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[type].SymbolSpriteID;
      let mut index1: i32 =  -1;
      if (att == 0)
        index1 = this.game.TempCombat.DefenderRegime;
      if (att == 1)
        index1 = this.game.TempCombat.AttackerRegime;
      if (index1 > -1)
      {
        if (this.game.Data.RegimeObj[index1].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[type].ExtraCounter;
          for (let mut index2: i32 =  0; index2 <= extraCounter; index2 += 1)
          {
            if (this.game.Data.SFTypeObj[type].ExtraCode[index2] == this.game.Data.RegimeObj[index1].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[type].ExtraSymbolSpriteID[index2];
          }
        }
        else
        {
          int index3;
          if (this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index3].ISFNr].People].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[type].ExtraCounter;
            for (let mut index4: i32 =  0; index4 <= extraCounter; index4 += 1)
            {
              if (this.game.Data.SFTypeObj[type].ExtraCode[index4] == this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index3].ISFNr].People].ExtraGraphicUse)
                symbolSpriteId = this.game.Data.SFTypeObj[type].ExtraSymbolSpriteID[index4];
            }
          }
        }
      }
      return symbolSpriteId;
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.TAid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList1Id)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num3 > -1)
              {
                this.detailnr1 = num3;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList2Id)
            {
              let mut num4: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num4 > -1)
              {
                this.detailnr2 = num4;
                this.detailnr4 = -1;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList3Id)
            {
              let mut num5: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num5 > -1)
              {
                this.detailnr3 = num5;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList4Id)
            {
              let mut num6: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num6 > -1)
              {
                this.detailnr4 = num6;
                this.detailnr2 = -1;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.okid)
            {
              if ( this.game.Data.RuleVar[839] == 1.0)
              {
                this.game.EditObj.PopupValue = 7;
                windowReturnClass.AddCommand(5, 10);
              }
              else
              {
                windowReturnClass.AddCommand(1, 84);
                windowReturnClass.AddCommand(2, 84);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
