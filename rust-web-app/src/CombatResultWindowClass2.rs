// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatResultWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class CombatResultWindowClass2 : WindowClass
  {
     okid: i32;
     tbacknr: i32;
     typeid: i32;
     oktextid: i32;
     detailid: i32;
     noteid: i32;
     note2id: i32;
     cloudid: i32;
     Pic1Id: i32;
     TAid: i32;
     showdetail: i32;
     DateTime lasttime;
     DateTime lastAnimTime;
     FromMessage: i32;
     bool ForwardKey;
     bool LastDrawAfterEnd;
     Hn: i32;
     resolveId: i32;
     int[] StartRdn;
     int[] StartEntr;
     int[] StartMor;
     int[] StartXp;
     useWidth: i32;
     useHeight: i32;
     useZoom: i32;
     int[,,,] crm;
     bool crmSet;
     attPage: i32;
     maxAttPage: i32;
     defPage: i32;
     maxDefPage: i32;
     slotCulture: i32;
     slotChar: i32;
     slotCharSkill: i32;
     slotSkillType: i32;
     slotUnitFeats: i32;
     tabid: Vec<i32>;
     int[] tabup;
     int[] tabdown;
     zoom1id: i32;
     zoom0id: i32;
     bool playBattle;
     mesId: i32;
     roundId: i32;
     playId: i32;
     crmSetOnRound: i32;
     SimpleList animList;
     bufferBitmap: Bitmap;

    pub CombatResultWindowClass2(ref tGame: GameClass, thn: i32)
      : base(ref tGame, 1, 1, 8)
    {
      this.StartRdn = new int[2];
      this.StartEntr = new int[2];
      this.StartMor = new int[2];
      this.StartXp = new int[2];
      this.crm = new int[2, 2, 2, 2];
      this.tabid = new int[2, 1000];
      this.tabup = new int[2];
      this.tabdown = new int[2];
    }

    pub CombatResultWindowClass2(ref tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
      : base(ref tGame, tUseWidth, tUseHeight, 8)
    {
      this.StartRdn = new int[2];
      this.StartEntr = new int[2];
      this.StartMor = new int[2];
      this.StartXp = new int[2];
      this.crm = new int[2, 2, 2, 2];
      this.tabid = new int[2, 1000];
      this.tabup = new int[2];
      this.tabdown = new int[2];
      this.slotCulture = tGame.HandyFunctionsObj.GetStringListByID(tGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      this.slotUnitFeats = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 402, 0, 0));
      this.slotChar = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
      this.slotCharSkill = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 199, 0, 0));
      this.slotSkillType = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 203, 0, 0));
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.lastAnimTime = DateAndTime.Now;
      this.useWidth = tUseWidth;
      this.useHeight = tUseHeight;
      this.game.EditObj.DoCardSlot = -1;
      this.showdetail = !this.game.EditObj.CombatTextual ? 0 : 1;
      this.animList = SimpleList::new();
      this.attPage = 0;
      this.maxAttPage = 0;
      this.defPage = 0;
      this.maxDefPage = 0;
      this.crmSet = false;
      this.useZoom = 1;
      this.dostuff();
    }

    pub fn HandleToolTip(x: i32, y: i32)
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

    pub GetUnitDescription: String(unr: i32)
    {
      let mut uslot: i32 =  this.game.TempCombat.FindUSlot(unr);
      unitDescription: String;
      if (this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode == 1 & this.game.TempCombat.UList[uslot].URetreated < 1)
        unitDescription = !(this.game.TempCombat.BattleEnded > 0 & this.game.HandyFunctionsObj.GetPowerPtsAbsolute(unr) < 1) ? "RETREATING" : "OVERRUN".to_owned();
      else if (this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode == 1)
        unitDescription = "RETREATED".to_owned();
      else if (this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode >= 2 & this.game.TempCombat.UList[uslot].URetreatMode < 5)
        unitDescription = "OUT OF AP";
      else if (!this.game.TempCombat.UList[uslot].UBreaks & this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode == 5)
        unitDescription = "PANICKED".to_owned();
      else if (this.game.TempCombat.UList[uslot].UBreaks)
        unitDescription = "BROKEN".to_owned();
      else if (this.game.TempCombat.UList[uslot].UBreaks & this.game.TempCombat.UList[uslot].URetreatMode == 5)
        unitDescription = "BROKEN".to_owned();
      else if (this.game.TempCombat.UList[uslot].URetreatMode == 0 & this.game.TempCombat.BattleEnded > 0 & this.game.TempCombat.UList[uslot].URetreated > 0 & this.game.TempCombat.CombatType == 3 & this.game.TempCombat.UList[uslot].Uattacker == 1)
        unitDescription = "OUT OF AP";
      else if (this.game.TempCombat.UList[uslot].URetreatMode == 0 & this.game.TempCombat.BattleEnded > 0 & this.game.TempCombat.UList[uslot].URetreated > 0)
        unitDescription = !(this.game.TempCombat.CombatType == 5 | this.game.TempCombat.CombatType == 13) ? "SURRENDERED" : "HOLDING".to_owned();
      else if (this.game.TempCombat.BattleEnded > 0)
      {
        unitDescription = this.game.TempCombat.BattleEnded <= 0 ? Strings.Trim(Conversion.Str( (this.game.HandyFunctionsObj.GetLowestAp(this.game.TempCombat.UList[uslot].UNr) - this.game.TempCombat.UList[uslot].UApSpend))) + "AP" : Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetLowestAp(this.game.TempCombat.UList[uslot].UNr))) + "AP".to_owned();
        if (this.game.TempCombat.UList[uslot].Uattacker < 1)
          unitDescription = "HOLDING".to_owned();
      }
      else
      {
        unitDescription = this.game.TempCombat.BattleEnded <= 0 ? Strings.Trim(Conversion.Str( (this.game.HandyFunctionsObj.GetLowestAp(this.game.TempCombat.UList[uslot].UNr) - this.game.TempCombat.UList[uslot].UApSpend))) + "AP" : Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetLowestAp(this.game.TempCombat.UList[uslot].UNr))) + "AP".to_owned();
        if (this.game.TempCombat.UList[uslot].Uattacker < 1)
          unitDescription = "HOLDING".to_owned();
      }
      return unitDescription;
    }

    pub GetUnitColor: Color(unr: i32)
    {
      let mut uslot: i32 =  this.game.TempCombat.FindUSlot(unr);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      return !this.game.TempCombat.UList[uslot].UBreaks ? (!(this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode == 1) ? (!(this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode <= 2) ? (!(this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode > 2 & this.game.TempCombat.UList[uslot].URetreatMode < 5) ? (!(!this.game.TempCombat.UList[uslot].UBreaks & this.game.TempCombat.UList[uslot].URetreat > 0 & this.game.TempCombat.UList[uslot].URetreatMode == 5) ? (this.game.TempCombat.BattleEnded <= 0 ? Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue) : (this.game.TempCombat.UList[uslot].Uattacker >= 1 ? Color.FromArgb((int) byte.MaxValue, 100, 100, 100) : Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue))) : Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 215, 215)) : Color.FromArgb((int) byte.MaxValue, 215, 215, 215)) : Color.FromArgb((int) byte.MaxValue, 215, 215, 215)) : Color.FromArgb((int) byte.MaxValue, 215, 215, 215)) : Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 215, 215);
    }

    pub GetCombatDescription: String()
    {
      let mut num: i32 =  0;
      let mut ucounter: i32 =  this.game.TempCombat.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        if (this.game.TempCombat.UList[index].Uattacker == 1)
        {
          if (num == 0)
            num = 1;
          if (this.game.TempCombat.UList[index].URetreat == 0 & this.game.TempCombat.UList[index].UDead == 0)
            num = 2;
        }
      }
      combatDescription: String;
      if (this.game.TempCombat.CombatType == 11)
      {
        if (this.game.TempCombat.BattleEnded == 0)
          combatDescription = "AMBUSH ONGOING\r\nROUND " + this.game.TempCombat.CombatRound.ToString();
        else if (this.game.TempCombat.BattleEnded == 1)
          combatDescription = "AMBUSH ENDED";
        else if (this.game.TempCombat.BattleEnded == 2)
          combatDescription = "AMBUSH ENDED";
        else if (this.game.TempCombat.BattleEnded == 3)
          combatDescription = "AMBUSH ENDED";
      }
      else if (this.game.TempCombat.CombatType == 1 | this.game.TempCombat.CombatType == 2 | this.game.TempCombat.CombatType == 10 | this.game.TempCombat.CombatType == 9)
      {
        if (this.game.TempCombat.BattleEnded == 0)
          combatDescription = "BATTLE ONGOING\r\nROUND " + this.game.TempCombat.CombatRound.ToString();
        else if (this.game.TempCombat.BattleEnded == 1)
          combatDescription = "ATTACKER WON";
        else if (this.game.TempCombat.BattleEnded == 2)
          combatDescription = "DEFENDER HELD";
        else if (this.game.TempCombat.BattleEnded == 3)
          combatDescription = "STANDOFF".to_owned();
      }
      else if (this.game.TempCombat.CombatType == 3 | this.game.TempCombat.CombatType == 4)
      {
        if (this.game.TempCombat.BattleEnded == 0)
          combatDescription = "ARTILLERY BOMBARDMENT\r\nROUND " + this.game.TempCombat.CombatRound.ToString();
        else if (this.game.TempCombat.BattleEnded == 1)
          combatDescription = "END OF BOMBARDMENT";
        else if (this.game.TempCombat.BattleEnded == 2)
          combatDescription = "END OF BOMBARDMENT";
        else if (this.game.TempCombat.BattleEnded == 3)
          combatDescription = "END OF BOMBARDMENT";
      }
      else if (this.game.TempCombat.CombatType == 5 | this.game.TempCombat.CombatType == 6)
      {
        if (this.game.TempCombat.BattleEnded == 0)
          combatDescription = "AIRSTRIKE\r\nROUND " + this.game.TempCombat.CombatRound.ToString();
        else if (this.game.TempCombat.BattleEnded == 1)
          combatDescription = "END OF AIRSTRIKE";
        else if (this.game.TempCombat.BattleEnded == 2)
          combatDescription = "END OF AIRSTRIKE";
        else if (this.game.TempCombat.BattleEnded == 3)
          combatDescription = "END OF AIRSTRIKE";
      }
      else if (this.game.TempCombat.CombatType == 13)
      {
        if (this.game.TempCombat.BattleEnded == 0)
          combatDescription = "AIRRECON\r\nROUND " + this.game.TempCombat.CombatRound.ToString();
        else if (this.game.TempCombat.BattleEnded == 1)
          combatDescription = "END OF AIRRECON";
        else if (this.game.TempCombat.BattleEnded == 2)
          combatDescription = "END OF AIRRECON";
        else if (this.game.TempCombat.BattleEnded == 3)
          combatDescription = "END OF AIRRECON";
      }
      return combatDescription;
    }

    pub void DrawIndividual(
      Graphics g,
      tx: i32,
      ty: i32,
      iid: i32,
      tw: i32,
      th: i32,
      bool useColMod,
      colMod: Color,
      bool mirror)
    {
      let mut regimeNr: i32 =  -1;
      let mut islot: i32 =  this.game.TempCombat.FindISlot(iid);
      if (islot < 0)
        return;
      let mut isfNr: i32 =  this.game.TempCombat.IList[islot].ISFNr;
      let mut tv0: i32 =  this.game.Data.PeopleObj[this.game.Data.SFObj[isfNr].People].tv0;
      let mut type: i32 =  this.game.Data.SFObj[isfNr].Type;
      let mut iunr: i32 =  this.game.TempCombat.IList[islot].IUnr;
      if (this.game.TempCombat.IList[islot].IAttacker == 1)
        regimeNr = this.game.TempCombat.AttackerRegime;
      if (this.game.TempCombat.IList[islot].IAttacker < 1)
        regimeNr = this.game.TempCombat.DefenderRegime;
      objBitmap: Bitmap;
      if (this.game.Data.SFTypeObj[type].SFTypeVar[89] < 1)
      {
        if (this.game.Data.SFTypeObj[type].SFTypeVar[82] > 0)
        {
          let mut y1: i32 =  -4;
          objBitmap = new Bitmap(76, 76, PixelFormat.Format32bppPArgb);
          Graphics graphics = Graphics.FromImage((Image) objBitmap);
          graphics.Clear(Color.Transparent);
          bitmap: Bitmap;
          Rectangle rectangle1;
          Rectangle rectangle2;
          if (this.game.Data.SFTypeObj[type].artCode[0] == 1)
          {
            ref Graphics local1 = ref graphics;
            bitmap = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[type].SidewaysSpriteID);
            ref local2: Bitmap = ref bitmap;
            rectangle1 = Rectangle::new(0, 0, 76, 76);
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(0, y1, 76, 76);
            let mut destrect: &Rectangle = &rectangle2
            double r =  ( this.game.Data.SFTypeObj[type].artCode[1] /  byte.MaxValue);
            double g1 =  ( this.game.Data.SFTypeObj[type].artCode[2] /  byte.MaxValue);
            double b =  ( this.game.Data.SFTypeObj[type].artCode[3] /  byte.MaxValue);
            DrawMod.DrawSimplePart2ColouredNew(ref local1, ref local2, srcrect, destrect,  r,  g1,  b, 1f);
          }
          else
          {
            ref Graphics local3 = ref graphics;
            bitmap = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[type].SidewaysSpriteID);
            ref local4: Bitmap = ref bitmap;
            let mut y2: i32 =  y1;
            DrawMod.DrawSimple(ref local3, ref local4, 0, y2);
          }
          if (this.game.Data.SFTypeObj[type].artCode[5] == 1)
          {
            ref Graphics local5 = ref graphics;
            bitmap = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[type].SymbolColBigSprite2ID);
            ref local6: Bitmap = ref bitmap;
            rectangle2 = Rectangle::new(0, 0, 76, 76);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, y1, 76, 76);
            let mut destrect: &Rectangle = &rectangle1
            double r =  ( this.game.Data.SFTypeObj[type].artCode[6] /  byte.MaxValue);
            double g2 =  ( this.game.Data.SFTypeObj[type].artCode[7] /  byte.MaxValue);
            double b =  ( this.game.Data.SFTypeObj[type].artCode[8] /  byte.MaxValue);
            double a =  ( this.game.Data.SFTypeObj[type].artCode[9] /  byte.MaxValue);
            DrawMod.DrawSimplePart2ColouredNew(ref local5, ref local6, srcrect, destrect,  r,  g2,  b,  a);
          }
          graphics.Dispose();
        }
        else
          objBitmap = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[type].SymbolSpriteID, this.useZoom);
      }
      else
      {
        bool isMilitia = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[iunr].Historical].GiveHisVarValue(11) > 0;
        let mut integer: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.slotCulture].GetData(0, tv0, 1));
        objBitmap = this.game.CustomBitmapObj.DrawSFTypeGraphic(type, isMilitia, integer, regimeNr, this.game.TempCombat.IList[islot].IUnr);
      }
      if (Information.IsNothing( objBitmap))
        return;
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      let mut w: i32 =  tw;
      let mut h: i32 =  th;
      let mut width: i32 =  objBitmap.Width;
      let mut height: i32 =  objBitmap.Height;
      if (width > w | height > h)
      {
        if ( width /  w >  height /  h)
        {
          float num3 =  w /  width;
          let mut num4: i32 =  (int) Math.Round( ( h -  height * num3));
          num2 += (int) Math.Round( num4 / 2.0);
          h -= num4;
        }
        else
        {
          float num5 =  h /  height;
          let mut num6: i32 =  (int) Math.Round( ( w -  width * num5));
          num1 += (int) Math.Round( num6 / 2.0);
          w -= num6;
        }
        if (mirror)
        {
          Matrix matrix = Matrix::new();
          matrix.Scale(-1f, 1f);
          matrix.Translate( -(2 * (tx + num1) + w), 0.0f);
          g.Transform = matrix;
        }
        if (useColMod)
          DrawMod.DrawScaledColorized2(ref g, ref objBitmap, tx + num1, ty + num2, w, h, objBitmap.Width, objBitmap.Height,  colMod.R /  byte.MaxValue,  colMod.G /  byte.MaxValue,  colMod.B /  byte.MaxValue,  colMod.A /  byte.MaxValue);
        else
          DrawMod.DrawScaled(ref g, ref objBitmap, tx + num1, ty + num2, w, h, true);
        if (!mirror)
          return;
        g.ResetTransform();
      }
      else
      {
        if (this.game.Data.SFTypeObj[type].SFTypeVar[89] < 1)
        {
          if (this.useZoom == 1)
          {
            tx += 4;
            ty += 4;
          }
          else
          {
            tx += 2;
            ty += 2;
          }
        }
        if (mirror)
        {
          Matrix matrix = Matrix::new();
          matrix.Scale(-1f, 1f);
          matrix.Translate( -(2 * tx + objBitmap.Width + (w - width)), 0.0f);
          g.Transform = matrix;
        }
        if (useColMod)
          DrawMod.Draw(ref g, ref objBitmap, tx + num1 + (int) Math.Round( (w - width) / 2.0), ty + num2 + (int) Math.Round( (h - height) / 2.0),  colMod.R /  byte.MaxValue - 1f,  colMod.G /  byte.MaxValue - 1f,  colMod.B /  byte.MaxValue - 1f,  colMod.A /  byte.MaxValue);
        else
          DrawMod.DrawSimple(ref g, ref objBitmap, tx + num1 + (int) Math.Round( (w - width) / 2.0), ty + num2 + (int) Math.Round( (h - height) / 2.0));
        if (!mirror)
          return;
        g.ResetTransform();
      }
    }

    pub fn dostuff(bool crmAlreadySet = false)
    {
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.playId > 0)
      {
        this.RemoveSubPart(this.playId);
        this.playId = 0;
      }
      if (this.roundId > 0)
      {
        this.RemoveSubPart(this.roundId);
        this.roundId = 0;
      }
      if (this.zoom1id > 0)
      {
        this.RemoveSubPart(this.zoom1id);
        this.zoom1id = 0;
      }
      if (this.zoom0id > 0)
      {
        this.RemoveSubPart(this.zoom0id);
        this.zoom0id = 0;
      }
      if (this.mesId > 0)
      {
        this.RemoveSubPart(this.mesId);
        this.mesId = 0;
      }
      if (this.typeid > 0)
      {
        this.RemoveSubPart(this.typeid);
        this.typeid = 0;
      }
      if (this.TAid > 0)
      {
        this.RemoveSubPart(this.TAid);
        this.TAid = 0;
      }
      if (this.detailid > 0)
      {
        this.RemoveSubPart(this.detailid);
        this.detailid = 0;
      }
      if (this.resolveId > 0)
      {
        this.RemoveSubPart(this.resolveId);
        this.resolveId = 0;
      }
      let mut index1: i32 =  0;
      do
      {
        if (this.tabid[0, index1] > 0)
        {
          this.RemoveSubPart(this.tabid[0, index1]);
          this.tabid[0, index1] = 0;
        }
        if (this.tabid[1, index1] > 0)
        {
          this.RemoveSubPart(this.tabid[1, index1]);
          this.tabid[1, index1] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 999);
      if (this.tabup[0] > 0)
      {
        this.RemoveSubPart(this.tabup[0]);
        this.tabup[0] = 0;
      }
      if (this.tabdown[0] > 0)
      {
        this.RemoveSubPart(this.tabdown[0]);
        this.tabdown[0] = 0;
      }
      if (this.tabup[1] > 0)
      {
        this.RemoveSubPart(this.tabup[1]);
        this.tabup[1] = 0;
      }
      if (this.tabdown[1] > 0)
      {
        this.RemoveSubPart(this.tabdown[1]);
        this.tabdown[1] = 0;
      }
      SizeF sizeF1 = SizeF::new();
      Color.FromArgb(160, 230, 210, 190);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(this.useWidth, this.useHeight, -1);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref graphics, 25, 40, this.useWidth - 50, this.useHeight - 40);
      if (!Information.IsNothing( this.BackBitmap))
      {
        this.BackBitmap.Dispose();
        this.BackBitmap = (Bitmap) null;
      }
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      let mut num1: i32 =  (int) Math.Round(Math.Floor( (this.useWidth - 1280) / 2.0));
      let mut int32: i32 =  Convert.ToInt32(Math.Floor(new Decimal(this.useHeight - 768)));
      Rectangle rectangle1 = Rectangle::new(35, 160, 595 + num1, 555 + int32);
      Rectangle rectangle2 = Rectangle::new(this.useWidth - (638 + num1), 160, 595 + num1, 555 + int32);
      Rectangle rectangle3 = Rectangle::new(35, 12, 370, 140);
      Rectangle rectangle4 = Rectangle::new(this.useWidth - 413, 12, 370, 140);
      Rectangle rectangle5 = Rectangle::new((int) Math.Round( this.useWidth / 2.0) - 185, 0, 371, 90);
      num2: i32;
      num3: i32;
      num4: i32;
      num5: i32;
      num6: i32;
      num7: i32;
      num8: i32;
      num9: i32;
      num10: i32;
      if (this.useZoom == 1)
      {
        num2 = 85;
        num3 = (int) Math.Round(Math.Floor( (555 + int32) /  num2));
        num4 = (int) Math.Round(Math.Floor( (595 + num1) /  num2));
        num5 = (int) Math.Round( (rectangle1.Width - num4 * num2) / 2.0);
        num6 = (int) Math.Round( (rectangle1.Height - num3 * num2) / 2.0);
        num7 = 4;
        num8 = 4;
        num9 = 76;
        num10 = 76;
      }
      else if (this.useZoom == 0)
      {
        num2 = 43;
        num3 = (int) Math.Round(Math.Floor( (555 + int32) /  num2));
        num4 = (int) Math.Round(Math.Floor( (595 + num1) /  num2));
        num5 = (int) Math.Round( (rectangle1.Width - num4 * num2) / 2.0);
        num6 = (int) Math.Round( (rectangle1.Height - num3 * num2) / 2.0);
        num7 = 2;
        num8 = 2;
        num9 = 38;
        num10 = 38;
      }
      if (!this.crmSet | this.crmSetOnRound < this.game.TempCombat.CombatRound)
      {
        let mut num11: i32 =  0;
        let mut num12: i32 =  0;
        let mut val2: i32 =  0;
        let mut val1: i32 =  0;
        this.crmSet = true;
        this.crmSetOnRound = this.game.TempCombat.CombatRound;
        bool[] flagArray = new bool[this.game.TempCombat.UCounter + 1];
        let mut ucounter1: i32 =  this.game.TempCombat.UCounter;
        for (let mut index2: i32 =  0; index2 <= ucounter1; index2 += 1)
        {
          let mut num13: i32 =  3;
          flagArray[index2] =  this.game.Data.RuleVar[431] <= 0.0;
          if (this.game.Data.MapObj[0].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].MaxRecon >= 0)
            flagArray[index2] = true;
          let mut icounter: i32 =  this.game.TempCombat.ICounter;
          num14: i32;
          for (let mut index3: i32 =  0; index3 <= icounter; index3 += 1)
          {
            if (this.game.TempCombat.IList[index3].IUnr == this.game.TempCombat.UList[index2].UNr)
            {
              bool flag = true;
              if ( this.game.Data.RuleVar[431] > 0.0)
              {
                flag = false;
                if (this.game.TempCombat.IList[index3].IvisibleFromRound <= this.game.TempCombat.CombatRound | this.game.Data.UnitObj[this.game.TempCombat.IList[index3].IUnr].Regime == this.game.Data.Turn)
                {
                  flag = true;
                  flagArray[index2] = true;
                }
              }
              if (!flagArray[index2])
                flag = false;
              if (flag)
              {
                num13 += 1;
                if (this.game.TempCombat.IList[index3].IunitFeatStart > 0)
                  num13 += 1;
              }
              else
              {
                num14 += 1;
                if (this.game.TempCombat.IList[index3].IunitFeatStart > 0)
                  num14 += 1;
              }
            }
          }
          if (flagArray[index2])
          {
            let mut num15: i32 =  (int) Math.Round(Math.Ceiling( num13 /  num4));
            if (this.game.TempCombat.UList[index2].Uattacker == 1)
              num11 += num15;
            else
              num12 += num15;
          }
          let mut num16: i32 =  (int) Math.Round(Math.Ceiling( (num14 + num13) /  num4));
          if (this.game.TempCombat.UList[index2].Uattacker == 1)
            val1 += num16;
          else
            val2 += num16;
        }
        let mut num17: i32 =  Math.Max(val1, val2) + (this.game.TempCombat.UCounter + 1);
        let mut num18: i32 =  num4 + 1;
        this.crm = new int[2, num18 + 1, num17 + (num3 + 2) + 1, 5];
        num4 = num18 - 1;
        let mut index4: i32 =  0;
        let mut index5: i32 =  0;
        let mut index6: i32 =  -1;
        let mut index7: i32 =  -1;
        let mut ucounter2: i32 =  this.game.TempCombat.UCounter;
        for (let mut index8: i32 =  0; index8 <= ucounter2; index8 += 1)
        {
          let mut index9: i32 =  0;
          if (this.game.TempCombat.UList[index8].Uattacker == 1)
            index9 = 1;
          if (flagArray[index8])
          {
            if (index9 == 1)
            {
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 1;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 1;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 2;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 2;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 5;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 5;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
            }
            else
            {
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 5;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 5;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 2;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 2;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 1)
                index6 += 1;
              else
                index7 += 1;
              if (index6 >= num4)
              {
                index4 += 1;
                index6 = 0;
              }
              if (index7 >= num4)
              {
                index5 += 1;
                index7 = 0;
              }
              if (index9 == 1)
                this.crm[index9, index6, index4, 0] = 1;
              if (index9 == 1)
                this.crm[index9, index6, index4, 1] = this.game.TempCombat.UList[index8].UNr;
              if (index9 == 0)
                this.crm[index9, index7, index5, 0] = 1;
              if (index9 == 0)
                this.crm[index9, index7, index5, 1] = this.game.TempCombat.UList[index8].UNr;
            }
            let mut icounter1: i32 =  this.game.TempCombat.ICounter;
            for (let mut index10: i32 =  0; index10 <= icounter1; index10 += 1)
            {
              if (this.game.TempCombat.IList[index10].IUnr == this.game.TempCombat.UList[index8].UNr && this.game.TempCombat.IList[index10].IunitFeatStart > 0)
              {
                bool flag = true;
                if ( this.game.Data.RuleVar[431] > 0.0)
                {
                  flag = false;
                  if (this.game.TempCombat.IList[index10].IvisibleFromRound <= this.game.TempCombat.CombatRound | this.game.Data.UnitObj[this.game.TempCombat.IList[index10].IUnr].Regime == this.game.Data.Turn)
                    flag = true;
                }
                if (flag)
                {
                  if (index9 == 1)
                    index6 += 1;
                  else
                    index7 += 1;
                  if (index6 >= num4)
                  {
                    index4 += 1;
                    index6 = 0;
                  }
                  if (index7 >= num4)
                  {
                    index5 += 1;
                    index7 = 0;
                  }
                  if (index9 == 1)
                    this.crm[index9, index6, index4, 0] = 4;
                  if (index9 == 1)
                    this.crm[index9, index6, index4, 1] = this.game.TempCombat.IList[index10].IID;
                  if (index9 == 0)
                    this.crm[index9, index7, index5, 0] = 4;
                  if (index9 == 0)
                    this.crm[index9, index7, index5, 1] = this.game.TempCombat.IList[index10].IID;
                }
              }
            }
            let mut icounter2: i32 =  this.game.TempCombat.ICounter;
            for (let mut index11: i32 =  0; index11 <= icounter2; index11 += 1)
            {
              if (this.game.TempCombat.IList[index11].IUnr == this.game.TempCombat.UList[index8].UNr)
              {
                bool flag = true;
                if ( this.game.Data.RuleVar[431] > 0.0)
                {
                  flag = false;
                  if (this.game.TempCombat.IList[index11].IvisibleFromRound <= this.game.TempCombat.CombatRound | this.game.Data.UnitObj[this.game.TempCombat.IList[index11].IUnr].Regime == this.game.Data.Turn)
                    flag = true;
                }
                if (flag)
                {
                  if (index9 == 1)
                    index6 += 1;
                  else
                    index7 += 1;
                  if (index6 >= num4)
                  {
                    index4 += 1;
                    index6 = 0;
                  }
                  if (index7 >= num4)
                  {
                    index5 += 1;
                    index7 = 0;
                  }
                  if (index9 == 1)
                    this.crm[index9, index6, index4, 0] = 3;
                  if (index9 == 1)
                    this.crm[index9, index6, index4, 1] = this.game.TempCombat.IList[index11].IID;
                  if (index9 == 0)
                    this.crm[index9, index7, index5, 0] = 3;
                  if (index9 == 0)
                    this.crm[index9, index7, index5, 1] = this.game.TempCombat.IList[index11].IID;
                }
              }
            }
            if (index9 == 1)
              index4 += 1;
            else
              index5 += 1;
            if (index9 == 1)
              index6 = -1;
            else
              index7 = -1;
          }
        }
        let mut num19: i32 =  (int) Math.Round(Math.Ceiling( (index4 + 1) /  num3) - 1.0);
        if (num19 < 0)
          num19 = 0;
        this.attPage = 0;
        this.maxAttPage = num19;
        let mut num20: i32 =  (int) Math.Round(Math.Ceiling( index5 /  num3) - 1.0);
        if (num20 < 0)
          num20 = 0;
        this.defPage = 0;
        this.maxDefPage = num20;
        let mut num21: i32 =  index5;
        for (let mut index12: i32 =  0; index12 <= num21; index12 += 1)
        {
          numArray: Vec<i32> = new int[num4 + 1, 5];
          let mut num22: i32 =  num4 - 1;
          for (let mut index13: i32 =  0; index13 <= num22; index13 += 1)
          {
            let mut index14: i32 =  0;
            do
            {
              numArray[index13, index14] = this.crm[0, index13, index12, index14];
              index14 += 1;
            }
            while (index14 <= 4);
          }
          let mut num23: i32 =  num4 - 1;
          for (let mut index15: i32 =  0; index15 <= num23; index15 += 1)
          {
            let mut index16: i32 =  0;
            do
            {
              this.crm[0, index15, index12, index16] = numArray[num4 - 1 - index15, index16];
              index16 += 1;
            }
            while (index16 <= 4);
          }
        }
      }
      int[] numArray1 = new int[2];
      int[] numArray2 = new int[2];
      int[] numArray3 = new int[2];
      int[] numArray4 = new int[2];
      int[] numArray5 = new int[2];
      int[] numArray6 = new int[2];
      let mut num24: i32 =  0;
      do
      {
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut index17: i32 =  0; index17 <= icounter; index17 += 1)
        {
          if (this.game.TempCombat.IList[index17].IAttacker == num24)
          {
            bool flag = true;
            if ( this.game.Data.RuleVar[431] > 0.0)
            {
              flag = false;
              if (this.game.TempCombat.IList[index17].IvisibleFromRound <= this.game.TempCombat.CombatRound | this.game.Data.UnitObj[this.game.TempCombat.IList[index17].IUnr].Regime == this.game.Data.Turn)
                flag = true;
            }
            if (flag)
            {
              int[] numArray7 = numArray1;
              int[] numArray8 = numArray7;
              let mut index18: i32 =  num24;
              let mut index19: i32 =  index18;
              let mut num25: i32 =  numArray7[index18] + 1;
              numArray8[index19] = num25;
              if (this.game.TempCombat.IList[index17].IKilled > 0)
              {
                int[] numArray9 = numArray4;
                int[] numArray10 = numArray9;
                let mut index20: i32 =  num24;
                let mut index21: i32 =  index20;
                let mut num26: i32 =  numArray9[index20] + 1;
                numArray10[index21] = num26;
              }
              else if (this.game.TempCombat.IList[index17].IRetreat > 0 | this.game.TempCombat.IList[index17].IRetreated > 0)
              {
                int[] numArray11 = numArray2;
                int[] numArray12 = numArray11;
                let mut index22: i32 =  num24;
                let mut index23: i32 =  index22;
                let mut num27: i32 =  numArray11[index22] + 1;
                numArray12[index23] = num27;
              }
              else if (this.game.TempCombat.IList[index17].IKilled < 1 & this.game.TempCombat.IList[index17].IRetreat < 1)
              {
                int[] numArray13 = numArray3;
                int[] numArray14 = numArray13;
                let mut index24: i32 =  num24;
                let mut index25: i32 =  index24;
                let mut num28: i32 =  numArray13[index24] + 1;
                numArray14[index25] = num28;
              }
              else
              {
                int[] numArray15 = numArray5;
                int[] numArray16 = numArray15;
                let mut index26: i32 =  num24;
                let mut index27: i32 =  index26;
                let mut num29: i32 =  numArray15[index26] + 1;
                numArray16[index27] = num29;
              }
            }
            else
            {
              int[] numArray17 = numArray6;
              int[] numArray18 = numArray17;
              let mut index28: i32 =  num24;
              let mut index29: i32 =  index28;
              let mut num30: i32 =  numArray17[index28] + 1;
              numArray18[index29] = num30;
            }
          }
        }
        num24 += 1;
      }
      while (num24 <= 1);
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      let mut counter1: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonus.Counter;
      num31: i32;
      for (let mut index30: i32 =  0; index30 <= counter1; index30 += 1)
      {
        let mut tid: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonus.Id[index30];
        let mut tdata3: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonus.Data2[index30];
        if (simpleList2.FindNr(tid, tdata3: tdata3) == -1 & tdata3 >= 1)
        {
          let mut num32: i32 =  0;
          let mut val2: i32 =  0;
          let mut counter2: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonus.Counter;
          for (let mut index31: i32 =  0; index31 <= counter2; index31 += 1)
          {
            if (this.game.TempCombat.customCombatObj.logLeaderBonus.Id[index31] == tid & this.game.TempCombat.customCombatObj.logLeaderBonus.Data2[index31] == this.game.TempCombat.customCombatObj.logLeaderBonus.Data2[index30])
            {
              num32 += this.game.TempCombat.customCombatObj.logLeaderBonus.Weight[index31];
              val2 += this.game.TempCombat.customCombatObj.logLeaderBonus.Data1[index31];
            }
          }
          let mut tdata1: i32 =  (int) Math.Round( num32 /  Math.Max(1, val2));
          let mut num33: i32 =  0;
          num31 = 0;
          let mut counter3: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonusDef.Counter;
          for (let mut index32: i32 =  0; index32 <= counter3; index32 += 1)
          {
            if (this.game.TempCombat.customCombatObj.logLeaderBonusDef.Id[index32] == tid & this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data2[index32] == this.game.TempCombat.customCombatObj.logLeaderBonus.Data2[index30])
            {
              num33 += this.game.TempCombat.customCombatObj.logLeaderBonusDef.Weight[index32];
              num31 += this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data1[index32];
            }
          }
          let mut tdata2: i32 =  (int) Math.Round( num33 /  Math.Max(1, num31));
          if (tdata1 >= 0 | tdata2 >= 0)
            simpleList2.Add(tid, (int) Math.Round( (tdata1 + tdata2) / 2.0), tdata1, tdata2, tdata3, CheckExistence: false);
        }
      }
      let mut counter4: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonusDef.Counter;
      for (let mut index33: i32 =  0; index33 <= counter4; index33 += 1)
      {
        let mut tid: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonusDef.Id[index33];
        let mut tdata3: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data2[index33];
        if (simpleList2.FindNr(tid, tdata3: tdata3) == -1 & tdata3 > 0)
        {
          let mut num34: i32 =  0;
          let mut val2: i32 =  0;
          let mut counter5: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonus.Counter;
          for (let mut index34: i32 =  0; index34 <= counter5; index34 += 1)
          {
            if (this.game.TempCombat.customCombatObj.logLeaderBonus.Id[index34] == tid & this.game.TempCombat.customCombatObj.logLeaderBonus.Data2[index34] == this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data2[index33])
            {
              num34 += this.game.TempCombat.customCombatObj.logLeaderBonus.Weight[index34];
              val2 += this.game.TempCombat.customCombatObj.logLeaderBonus.Data1[index34];
            }
          }
          let mut tdata1: i32 =  (int) Math.Round( num34 /  Math.Max(1, val2));
          let mut num35: i32 =  0;
          num31 = 0;
          let mut counter6: i32 =  this.game.TempCombat.customCombatObj.logLeaderBonusDef.Counter;
          for (let mut index35: i32 =  0; index35 <= counter6; index35 += 1)
          {
            if (this.game.TempCombat.customCombatObj.logLeaderBonusDef.Id[index35] == tid & this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data2[index35] == this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data2[index33])
            {
              num35 += this.game.TempCombat.customCombatObj.logLeaderBonusDef.Weight[index35];
              num31 += this.game.TempCombat.customCombatObj.logLeaderBonusDef.Data1[index35];
            }
          }
          let mut tdata2: i32 =  (int) Math.Round( num35 /  Math.Max(1, num31));
          if (tdata1 > 0 | tdata2 > 0)
            simpleList2.Add(tid, (int) Math.Round( (tdata1 + tdata2) / 2.0), tdata1, tdata2, tdata3, CheckExistence: false);
        }
      }
      let mut counter7: i32 =  simpleList2.Counter;
      for (let mut index36: i32 =  0; index36 <= counter7; index36 += 1)
      {
        let mut tid: i32 =  simpleList2.Id[index36];
        if (simpleList1.FindNr(tid) == -1)
        {
          let mut tdata1: i32 =  0;
          let mut tdata2: i32 =  0;
          let mut counter8: i32 =  simpleList2.Counter;
          for (let mut index37: i32 =  0; index37 <= counter8; index37 += 1)
          {
            if (simpleList2.Id[index36] == simpleList2.Id[index37])
            {
              if (simpleList2.Data1[index37] > 0)
              {
                let mut num36: i32 =  (int) Math.Round( (tdata1 * simpleList2.Data1[index37]) / 100.0);
                tdata1 += simpleList2.Data1[index37] + num36;
              }
              if (simpleList2.Data2[index37] > 0)
              {
                let mut num37: i32 =  (int) Math.Round( (tdata2 * simpleList2.Data2[index37]) / 100.0);
                tdata2 += simpleList2.Data2[index37] + num37;
              }
            }
          }
          let mut num38: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList2.Id[index36], 6)));
          let mut tweight: i32 =  5;
          if (num38 == 4)
            tweight = 10;
          simpleList1.Add(tid, tweight, tdata1, tdata2, CheckExistence: false);
        }
      }
      simpleList2.ReverseSort();
      simpleList1.ReverseSort();
      bool flag1 = false;
      let mut counter9: i32 =  simpleList1.Counter;
      for (let mut index38: i32 =  0; index38 <= counter9; index38 += 1)
      {
        let mut num39: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index38], 6)));
        let mut num40: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index38], 25)));
        let mut id: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index38], 5)));
        if (id < 1 & num40 > 0)
          id = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index38], 26)));
        if (this.game.HandyFunctionsObj.GetRegimeByID(id) == this.game.TempCombat.AttackerRegime && num40 > 0)
          flag1 = true;
      }
      if (flag1)
        DrawMod.DrawBlockGradient2(ref graphics, 38, 46, this.useWidth - 82, this.useHeight - 64, Color.FromArgb(125, (int) byte.MaxValue, 0, 0), Color.FromArgb(0, (int) byte.MaxValue, 0, 0));
      ref Graphics local1 = ref graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBAR1);
      ref local2: Bitmap = ref bitmap;
      let mut x1: i32 =  rectangle3.X;
      let mut y1: i32 =  rectangle3.Y;
      DrawMod.DrawSimple(ref local1, ref local2, x1, y1);
      Matrix matrix1 = Matrix::new();
      matrix1.Scale(-1f, 1f);
      matrix1.Translate(0.0f, 0.0f);
      graphics.Transform = matrix1;
      ref Graphics local3 = ref graphics;
      bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBAR1);
      ref local4: Bitmap = ref bitmap;
      let mut x2: i32 =  -(rectangle4.X + BitmapStore.GetWidth(this.game.SE1_COMBATBAR1));
      let mut y2: i32 =  rectangle3.Y;
      DrawMod.DrawSimple(ref local3, ref local4, x2, y2);
      graphics.ResetTransform();
      let mut num41: i32 =  0;
      let mut num42: i32 =  0;
      let mut num43: i32 =  62;
      let mut w1: i32 =  (int) Math.Round( (100 * num43) / 140.0);
      let mut x3: i32 =  rectangle3.X;
      let mut num44: i32 =  rectangle3.Y + 87;
      let mut num45: i32 =  x3 + 100;
      let mut num46: i32 =  num41 + 100;
      let mut num47: i32 =  rectangle4.X + rectangle4.Width - (90 + w1 + 50);
      if (this.game.ScreenWidth < 1660)
      {
        num45 -= 90;
        num47 += 90;
      }
      let mut num48: i32 =  0;
      let mut num49: i32 =  0;
      let mut counter10: i32 =  simpleList1.Counter;
      Rectangle trect1;
      Rectangle trect2;
      for (let mut index39: i32 =  0; index39 <= counter10; index39 += 1)
      {
        let mut num50: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index39], 6)));
        let mut num51: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index39], 25)));
        let mut id: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index39], 5)));
        if (id < 1 & num51 > 0)
          id = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index39], 26)));
        let mut regimeById: i32 =  this.game.HandyFunctionsObj.GetRegimeByID(id);
        bool flag2 = false;
        if (regimeById == this.game.TempCombat.AttackerRegime)
        {
          if (num49 <= 2)
          {
            ref Graphics local5 = ref graphics;
            bitmap = this.game.CustomBitmapObj.DrawLeaderPortrait(simpleList1.Id[index39], w1, num43);
            ref local6: Bitmap = ref bitmap;
            let mut x4: i32 =  num45;
            let mut y3: i32 =  num44;
            DrawMod.DrawSimple(ref local5, ref local6, x4, y3);
            DrawMod.DrawBlock(ref graphics, num45 + w1, num44, 45, num43, 0, 0, 0, 128);
            DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, num45 - 6, num44, w1 + 51, num43, 10, 10, 10);
            str: String = ((int) Math.Round( simpleList1.Data1[index39] +  simpleList1.Data2[index39] / 2.0)).ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if ((int) Math.Round( simpleList1.Data1[index39] +  simpleList1.Data2[index39] / 2.0) > 0)
              str = "+" + str;
            tstring1: String = "HQ".to_owned();
            if (num51 < 1)
            {
              if (num50 == 3)
                tstring1 = "OHQ".to_owned();
              if (num50 == 4)
                tstring1 = "SHQ".to_owned();
              DrawMod.DrawTextColouredMarcCenter(ref graphics, tstring1, this.game.MarcFont4, num45 + w1 + 21, num44 + (int) Math.Round( num43 * 0.35) - 10, Color.LightGray);
            }
            else
            {
              tstring2: String = "KIA".to_owned();
              DrawMod.DrawTextColouredMarcCenter(ref graphics, tstring2, this.game.MarcFont4, num45 + w1 + 21, num44 + (int) Math.Round( num43 * 0.35) - 10, Color.Red);
            }
            DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + w1 + 21, num44 + (int) Math.Round( num43 * 0.35) + 10, Color.White);
            num31 = w1 + 55;
            num49 += 1;
          }
          else
            flag2 = true;
        }
        else if ( this.game.Data.MapObj[0].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].MaxRecon <  this.game.Data.RuleVar[55])
          flag2 = true;
        else if (num48 <= 2)
        {
          ref Graphics local7 = ref graphics;
          bitmap = this.game.CustomBitmapObj.DrawLeaderPortrait(simpleList1.Id[index39], w1, num43);
          ref local8: Bitmap = ref bitmap;
          let mut x5: i32 =  num47;
          let mut y4: i32 =  num44;
          DrawMod.DrawSimple(ref local7, ref local8, x5, y4);
          DrawMod.DrawBlock(ref graphics, num47 + w1, num44, 45, num43, 0, 0, 0, 128);
          DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, num47 - 6, num44, w1 + 51, num43, 10, 10, 10);
          str: String = ((int) Math.Round( simpleList1.Data1[index39] +  simpleList1.Data2[index39] / 2.0)).ToString() + "%";
          if (Operators.CompareString(str, "0%", false) == 0)
            str = "-";
          else if ((int) Math.Round( simpleList1.Data1[index39] +  simpleList1.Data2[index39] / 2.0) > 0)
            str = "+" + str;
          tstring: String = "HQ".to_owned();
          if (num50 == 3)
            tstring = "OHQ".to_owned();
          if (num50 == 4)
            tstring = "SHQ".to_owned();
          DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num47 + w1 + 21, num44 + (int) Math.Round( num43 * 0.35) + 10, Color.White);
          DrawMod.DrawTextColouredMarcCenter(ref graphics, tstring, this.game.MarcFont4, num47 + w1 + 21, num44 + (int) Math.Round( num43 * 0.35) - 10, Color.White);
          num31 = w1 + 55;
          num48 += 1;
        }
        else
          flag2 = true;
        let mut num52: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotChar].GetData(0, simpleList1.Id[index39], 6)));
        leaderName: String = this.game.EventRelatedObj.GetLeaderName(simpleList1.Id[index39], true);
        ttext: String = (num51 <= 0 ? this.game.EventRelatedObj.Helper_GetCharacterJobTitle(simpleList1.Id[index39]) + "\r\n" : "Leader was killed in combat.\r\n") + "-----------------\r\n" + "Skill rolls added on average " + simpleList1.Data1[index39].ToString() + "% to attack power of subordinate troops.\r\n" + "Skill rolls added on average " + simpleList1.Data2[index39].ToString() + "% to hitpoints of subordinate troops.\r\n";
        str1: String = "";
        let mut counter11: i32 =  simpleList2.Counter;
        for (let mut index40: i32 =  0; index40 <= counter11; index40 += 1)
        {
          if (simpleList2.Id[index40] == simpleList1.Id[index39] && simpleList2.Data1[index40] > 0 | simpleList2.Data2[index40] > 0)
          {
            let mut idValue: i32 =  simpleList2.Data3[index40];
            str1 = str1 + this.game.Data.StringListObj[this.slotSkillType].GetData(0, idValue, 1) + " skill rolls gave " + simpleList2.Data1[index40].ToString() + "% attack power bonus and " + simpleList2.Data2[index40].ToString() + "% hitpoints bonus." + "\r\n";
          }
        }
        if (str1.Length > 1)
          ttext = ttext + "-----------------\r\n" + str1;
        if (!flag2)
        {
          if (regimeById == this.game.TempCombat.AttackerRegime)
          {
            trect1 = Rectangle::new(num45, num44, num31, num43);
            trect2 = trect1;
            this.AddMouse(ref trect2, leaderName, ttext);
            num45 += num31;
            num46 += num31;
          }
          else
          {
            trect2 = Rectangle::new(num47, num44, num31, num43);
            trect1 = trect2;
            this.AddMouse(ref trect1, leaderName, ttext);
            num47 -= num31;
            num42 += num31;
          }
        }
      }
      for (let mut index41: i32 =  num48; index41 <= 2; index41 += 1)
      {
        DrawMod.DrawBlock(ref graphics, num47, num44, 45 + w1, num43, 0, 0, 0, 128);
        DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, num47 - 6, num44, w1 + 51, num43, 10, 10, 10);
        let mut num53: i32 =  w1 + 55;
        num47 -= num53;
        num42 += num53;
      }
      for (let mut index42: i32 =  num49; index42 <= 2; index42 += 1)
      {
        DrawMod.DrawBlock(ref graphics, num45, num44, 45 + w1, num43, 0, 0, 0, 128);
        DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, num45 - 6, num44, w1 + 51, num43, 10, 10, 10);
        let mut num54: i32 =  w1 + 55;
        num45 += num54;
        num46 += num54;
      }
      if (num42 > 0)
        num42 += 50 + w1;
      if (num42 > num46)
        num46 = num42;
      if (num46 > num42)
        ;
      let mut x6: i32 =  num47 + 40;
      let mut num55: i32 =  1;
      index43: i32;
      num56: i32;
      index44: i32;
      index45: i32;
      do
      {
        DrawMod.DrawBlock(ref graphics, num45, num44, 45, 62, 0, 0, 0, 128);
        let mut tid: i32 =  0;
        do
        {
          let mut num57: i32 =  0;
          let mut num58: i32 =  0;
          index43 = 0;
          let mut num59: i32 =  0;
          let mut num60: i32 =  0;
          let mut num61: i32 =  0;
          let mut num62: i32 =  0;
          let mut num63: i32 =  0;
          let mut num64: i32 =  0;
          num56 = 0;
          let mut num65: i32 =  0;
          let mut num66: i32 =  0;
          if (num55 == 1)
          {
            let mut nr1: i32 =  this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 1);
            if (nr1 > -1)
            {
              num58 = this.game.TempCombat.customCombatObj.otherBonus.Weight[nr1];
              num57 = this.game.TempCombat.customCombatObj.otherBonus.Data1[nr1];
            }
            let mut nr2: i32 =  this.game.TempCombat.customCombatObj.otherBonusDef.FindNr(tid, tdata2: 1);
            if (nr2 > -1)
            {
              num63 = this.game.TempCombat.customCombatObj.otherBonusDef.Weight[nr2];
              num62 = this.game.TempCombat.customCombatObj.otherBonusDef.Data1[nr2];
            }
            let mut nr3: i32 =  this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 2);
            if (nr3 > -1)
            {
              num59 = this.game.TempCombat.customCombatObj.otherBonus.Weight[nr3];
              index43 = this.game.TempCombat.customCombatObj.otherBonus.Data1[nr3];
            }
            let mut nr4: i32 =  this.game.TempCombat.customCombatObj.otherBonusDef.FindNr(tid, tdata2: 2);
            if (nr4 > -1)
            {
              num56 = this.game.TempCombat.customCombatObj.otherBonusDef.Weight[nr4];
              num64 = this.game.TempCombat.customCombatObj.otherBonusDef.Data1[nr4];
            }
          }
          if (num55 == 2)
          {
            let mut nr5: i32 =  this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 4);
            if (nr5 > -1)
            {
              num58 = this.game.TempCombat.customCombatObj.otherBonus.Weight[nr5];
              num57 = this.game.TempCombat.customCombatObj.otherBonus.Data1[nr5];
            }
            let mut nr6: i32 =  this.game.TempCombat.customCombatObj.otherBonusDef.FindNr(tid, tdata2: 4);
            if (nr6 > -1)
            {
              num63 = this.game.TempCombat.customCombatObj.otherBonusDef.Weight[nr6];
              num62 = this.game.TempCombat.customCombatObj.otherBonusDef.Data1[nr6];
            }
            let mut nr7: i32 =  this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 5);
            if (nr7 > -1)
            {
              num59 = this.game.TempCombat.customCombatObj.otherBonus.Weight[nr7];
              index43 = this.game.TempCombat.customCombatObj.otherBonus.Data1[nr7];
            }
            let mut nr8: i32 =  this.game.TempCombat.customCombatObj.otherBonusDef.FindNr(tid, tdata2: 5);
            if (nr8 > -1)
            {
              num56 = this.game.TempCombat.customCombatObj.otherBonusDef.Weight[nr8];
              num64 = this.game.TempCombat.customCombatObj.otherBonusDef.Data1[nr8];
            }
          }
          if (num55 >= 3)
          {
            let mut repCounter: i32 =  this.game.TempCombat.RepCounter;
            for (let mut index46: i32 =  0; index46 <= repCounter; index46 += 1)
            {
              let mut num67: i32 =  this.game.TempCombat.RepFrom[index46];
              if (num67 >= 10000 & this.game.TempCombat.RepType[index46] == 1 && this.game.TempCombat.IList[num67 - 10000].IAttacker == tid)
              {
                if (Strings.InStr(this.game.TempCombat.RepTitle[index46], "=>") > 0)
                {
                  let mut index47: i32 =  0;
                  do
                  {
                    switch (num55)
                    {
                      case 3:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Landscape", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          num57 += 1;
                          num58 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Concentric", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          index43 += 1;
                          num59 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Entrenchment", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          num60 += 1;
                          num61 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                          break;
                        }
                        break;
                      case 4:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Readiness", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          num57 += 1;
                          num58 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Experience", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          index43 += 1;
                          num59 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                          break;
                        }
                        break;
                      case 5:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Supply", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          num57 += 1;
                          num58 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Ammunition Mod", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          index43 += 1;
                          num59 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index47, 0, index46], "Fuel Mod", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index47, 1, index46], "?", false) != 0)
                        {
                          num60 += 1;
                          num61 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index47, 1, index46]));
                          break;
                        }
                        break;
                    }
                    index47 += 1;
                  }
                  while (index47 <= 60);
                }
                if (Strings.InStr(this.game.TempCombat.RepTitle[index46], "<=") > 0)
                {
                  let mut index48: i32 =  0;
                  do
                  {
                    switch (num55)
                    {
                      case 3:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Landscape", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num62 += 1;
                          num63 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Concentric", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num64 += 1;
                          num56 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Entrenchment", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num65 += 1;
                          num66 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                          break;
                        }
                        break;
                      case 4:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Readiness", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num62 += 1;
                          num63 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Experience", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num64 += 1;
                          num56 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                          break;
                        }
                        break;
                      case 5:
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Supply", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num57 += 1;
                          num58 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Ammunition Mod", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          index43 += 1;
                          num59 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                        }
                        if (Operators.CompareString(this.game.TempCombat.RepCom[index48, 3, index46], "Fuel Mod", false) == 0 & Operators.CompareString(this.game.TempCombat.RepCom[index48, 4, index46], "?", false) != 0)
                        {
                          num60 += 1;
                          num61 += (int) Math.Round(Conversion.Val(this.game.TempCombat.RepCom[index48, 4, index46]));
                          break;
                        }
                        break;
                    }
                    index48 += 1;
                  }
                  while (index48 <= 60);
                }
              }
            }
          }
          index44 = num57 <= 0 ? 0 : (int) Math.Round( num58 /  num57);
          let mut num68: i32 =  index43 <= 0 ? 0 : (int) Math.Round( num59 /  index43);
          let mut num69: i32 =  num60 <= 0 ? 0 : (int) Math.Round( num61 /  num60);
          let mut num70: i32 =  num62 <= 0 ? 0 : (int) Math.Round( num63 /  num62);
          num56 = num64 <= 0 ? 0 : (int) Math.Round( num56 /  num64);
          num66 = num65 <= 0 ? 0 : (int) Math.Round( num66 /  num65);
          if (num55 == 1)
          {
            let mut num71: i32 =  4;
            index45 = (int) Math.Round( (100 + index44) * ( (100 + num68) / 100.0) * ( (num70 + 100) / 100.0) * ( (num56 + 100) / 100.0)) - 100;
            str: String = index45.ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if (Operators.CompareString(Strings.Left(str, 1), "-", false) != 0 & Strings.Len(str) > 1)
              str = "+" + str;
            if (num57 == 0 & index43 == 0 & num62 == 0 & num64 == 0 & tid == 0)
              str = "?";
            ttext: String = "Posture Bonus to Attack Values : " + index44.ToString() + "%\r\n" + "Posture Bonus of Hitpoints : " + num70.ToString() + "%\r\n" + "Profile Bonus to Attack Values : " + num68.ToString() + "%\r\n";
            if (tid == 1)
            {
              ref Graphics local9 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local10: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num71 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(num45 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
            }
            if (tid == 1)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", "No bonus or penalty.");
            }
            if (tid == 0)
            {
              ref Graphics local11 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local12: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num71 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(x6 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect, destrect);
            }
            if (tid == 0)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, x6 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPORT MODIFIERS", "No bonus or penalty.");
            }
          }
          if (num55 == 2)
          {
            let mut num72: i32 =  30;
            index45 = (int) Math.Round( (100 + index44) * ( (100 + num68) / 100.0) * ( (num70 + 100) / 100.0) * ( (num56 + 100) / 100.0)) - 100;
            str: String = index45.ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if (Operators.CompareString(Strings.Left(str, 1), "-", false) != 0 & Strings.Len(str) > 1)
              str = "+" + str;
            if (this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 4) > -1 | this.game.TempCombat.customCombatObj.otherBonus.FindNr(tid, tdata2: 5) > -1 && num57 == 0 & index43 == 0 & num62 == 0 & num64 == 0 & tid == 0)
              str = "?";
            ttext: String = "Weapon Matrix Modifier for Attack Values : " + index44.ToString() + "%\r\n" + "Callibre Modifier to Attack Values : " + num68.ToString() + "%\r\n";
            if (tid == 1)
            {
              ref Graphics local13 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local14: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num72 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(num45 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect, destrect);
            }
            if (tid == 1)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", "No bonus or penalty.");
            }
            if (tid == 0)
            {
              ref Graphics local15 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local16: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num72 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(x6 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
            }
            if (tid == 0)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, x6 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "WEAPON MODIFIERS", "No bonus or penalty.");
            }
          }
          if (num55 == 3)
          {
            let mut num73: i32 =  38;
            index45 = (int) Math.Round( (100 + index44) * ( (100 + num68) / 100.0) * ( (num70 + 100) / 100.0) * ( (num56 + 100) / 100.0) * ( (num66 + 100) / 100.0) * ( (num69 + 100) / 100.0)) - 100;
            str: String = index45.ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if (Operators.CompareString(Strings.Left(str, 1), "-", false) != 0 & Strings.Len(str) > 1)
              str = "+" + str;
            if (num57 == 0 & index43 == 0 & num62 == 0 & num64 == 0 & tid == 0)
              str = "?";
            ttext: String = "Landscape Modifier of Attack Values : " + index44.ToString() + "%\r\n" + "Concentric Attack Modifier of Attack Values : " + num68.ToString() + "%\r\n" + "Landscape Modifier of Hitpoints : " + num70.ToString() + "%\r\n" + "Concentric Attack Modifier of Hitpoints : " + num56.ToString() + "%\r\n" + "Entrenchment Modifier of Hitpoints: " + num66.ToString() + "%\r\n";
            if (tid == 1)
            {
              ref Graphics local17 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local18: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num73 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(num45 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect, destrect);
            }
            if (tid == 1)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", "No bonus or penalty.");
            }
            if (tid == 0)
            {
              ref Graphics local19 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local20: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num73 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(x6 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local19, ref local20, srcrect, destrect);
            }
            if (tid == 0)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, x6 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TERRAIN MODIFIERS", "No bonus or penalty.");
            }
          }
          if (num55 == 4)
          {
            let mut num74: i32 =  33;
            index45 = (int) Math.Round( (100 + index44) * ( (100 + num68) / 100.0) * ( (num70 + 100) / 100.0) * ( (num56 + 100) / 100.0)) - 100;
            str: String = index45.ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if (Operators.CompareString(Strings.Left(str, 1), "-", false) != 0 & Strings.Len(str) > 1)
              str = "+" + str;
            if (num57 == 0 & index43 == 0 & num62 == 0 & num64 == 0 & tid == 0)
              str = "?";
            ttext: String = "Readiness Modifier of Attack Values: " + index44.ToString() + "%\r\n" + "Experience Modifier of Attack Values : " + num68.ToString() + "%\r\n" + "Readiness Modifier of Hitpoints: " + num70.ToString() + "%\r\n" + "Experience Modifier of Hitpoints : " + num56.ToString() + "%\r\n";
            if (tid == 1)
            {
              ref Graphics local21 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local22: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num74 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(num45 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local21, ref local22, srcrect, destrect);
            }
            if (tid == 1)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", "No bonus or penalty.");
            }
            if (tid == 0)
            {
              ref Graphics local23 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local24: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num74 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(x6 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local23, ref local24, srcrect, destrect);
            }
            if (tid == 0)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, x6 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "TROOP MODIFIERS", "No bonus or penalty.");
            }
          }
          if (num55 == 5)
          {
            let mut num75: i32 =  37;
            index45 = (int) Math.Round( (100 + index44) * ( (100 + num68) / 100.0) * ( (num70 + 100) / 100.0) * ( (num56 + 100) / 100.0) * ( (num66 + 100) / 100.0) * ( (num69 + 100) / 100.0)) - 100;
            str: String = index45.ToString() + "%";
            if (Operators.CompareString(str, "0%", false) == 0)
              str = "-";
            else if (Operators.CompareString(Strings.Left(str, 1), "-", false) != 0 & Strings.Len(str) > 1)
              str = "+" + str;
            if (num57 == 0 & index43 == 0 & num62 == 0 & num64 == 0 & num60 == 0 & num65 == 0 & tid == 0)
              str = "?";
            ttext: String = "Supply Modifier of Attack Values: " + index44.ToString() + "%\r\n" + "Ammo Modifier of Attack Values : " + num68.ToString() + "%\r\n" + "Fuel Modifier of Attack Values : " + num69.ToString() + "%\r\n" + "Supply Modifier of Hitpoints: " + num70.ToString() + "%\r\n" + "Ammo Modifier of Hitpoints : " + num56.ToString() + "%\r\n" + "Fuel Modifier  of Hitpoints : " + num66.ToString() + "%\r\n";
            if (tid == 1)
            {
              ref Graphics local25 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local26: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num75 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(num45 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local25, ref local26, srcrect, destrect);
            }
            if (tid == 1)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, num45 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 1)
            {
              trect2 = Rectangle::new(num45, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", "No bonus or penalty.");
            }
            if (tid == 0)
            {
              ref Graphics local27 = ref graphics;
              bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local28: Bitmap = ref bitmap;
              trect2 = Rectangle::new(num75 * 42, 0, 42, 32);
              let mut srcrect: &Rectangle = &trect2
              trect1 = Rectangle::new(x6 + 2, num44 + 3, 42, 32);
              let mut destrect: &Rectangle = &trect1
              DrawMod.DrawSimplePart2(ref local27, ref local28, srcrect, destrect);
            }
            if (tid == 0)
              DrawMod.DrawTextColouredMarcCenter(ref graphics, str, this.game.MarcFont16, x6 + 22, num44 + 34, Color.White);
            if (Operators.CompareString(str, "?", false) != 0 & Operators.CompareString(str, "-", false) != 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", ttext);
            }
            if (Operators.CompareString(str, "?", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", "No details available.");
            }
            if (Operators.CompareString(str, "-", false) == 0 & tid == 0)
            {
              trect2 = Rectangle::new(x6, num44, 45, 60);
              trect1 = trect2;
              this.AddMouse(ref trect1, "SUPPLY MODIFIERS", "No bonus or penalty.");
            }
          }
          tid += 1;
        }
        while (tid <= 1);
        DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, num45, num44, 45, 62, 10, 10, 10);
        DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, x6, num44, 45, 62, 10, 10, 10);
        num45 += 46;
        x6 -= 46;
        num55 += 1;
      }
      while (num55 <= 5);
      let mut index49: i32 =  0;
      do
      {
        if (index49 == 0)
          index44 = this.game.TempCombat.DefenderRegime;
        if (index49 == 1)
          index44 = this.game.TempCombat.AttackerRegime;
        if (index44 == -1)
          index44 = 1;
        let mut red: i32 =  this.game.Data.RegimeObj[index44].Red;
        let mut green: i32 =  this.game.Data.RegimeObj[index44].Green;
        let mut blue: i32 =  this.game.Data.RegimeObj[index44].Blue;
        let mut red2: i32 =  this.game.Data.RegimeObj[index44].Red2;
        let mut green2: i32 =  this.game.Data.RegimeObj[index44].Green2;
        let mut blue2: i32 =  this.game.Data.RegimeObj[index44].Blue2;
        if (((this.game.TempCombat.BattleEnded <= 0 ? 1 : 0) | 1) != 0)
        {
          Rectangle rectangle6;
          if (index49 == 1)
            rectangle6 = Rectangle::new(rectangle3.X + 100, rectangle3.Y + 44, 240, 30);
          if (index49 == 0)
            rectangle6 = Rectangle::new(rectangle4.X + rectangle4.Width - 340, rectangle4.Y + 44, 240, 30);
          color1: Color = Color.FromArgb((int) byte.MaxValue, 55, 155, 55);
          color2: Color = Color.FromArgb((int) byte.MaxValue, 55, 105, 155);
          color3: Color = Color.FromArgb((int) byte.MaxValue, 105, 105, 105);
          color4: Color = Color.FromArgb((int) byte.MaxValue, 155, 55, 55);
          let mut num76: i32 =  rectangle6.Width - 8;
          if (numArray1[index49] > 0)
          {
            if (index49 == 1)
            {
              let mut num77: i32 =  4;
              let mut num78: i32 =  (int) Math.Round( (num76 * numArray3[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num77, rectangle6.Y + 4, num78, rectangle6.Height - 8, (int) color1.R, (int) color1.G, (int) color1.B, (int) color1.A);
              trect2 = Rectangle::new(rectangle6.X + num77, rectangle6.Y + 4, num78, rectangle6.Height - 8);
              let mut trect3: &Rectangle = &trect2
              this.AddMouse(ref trect3, "", "Attacker still has " + numArray3[index49].ToString() + " sub units fighting.");
              let mut num79: i32 =  num77 + num78;
              let mut num80: i32 =  (int) Math.Round( (num76 * numArray2[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num79, rectangle6.Y + 4, num80, rectangle6.Height - 8, (int) color2.R, (int) color2.G, (int) color2.B, (int) color2.A);
              trect2 = Rectangle::new(rectangle6.X + num79, rectangle6.Y + 4, num80, rectangle6.Height - 8);
              let mut trect4: &Rectangle = &trect2
              this.AddMouse(ref trect4, "", "Attacker has " + numArray2[index49].ToString() + " sub units that are retreating, or have retreated.");
              let mut num81: i32 =  num79 + num80;
              let mut num82: i32 =  (int) Math.Round( (num76 * numArray5[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num81, rectangle6.Y + 4, num82, rectangle6.Height - 8, (int) color3.R, (int) color3.G, (int) color3.B, (int) color3.A);
              trect2 = Rectangle::new(rectangle6.X + num81, rectangle6.Y + 4, num82, rectangle6.Height - 8);
              let mut trect5: &Rectangle = &trect2
              this.AddMouse(ref trect5, "", "Attacker has " + numArray5[index49].ToString() + " sub units that are neither fighting, nor retreating, nor killed.");
              let mut num83: i32 =  num81 + num82;
              index44 = (int) Math.Round( (num76 * numArray4[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num83, rectangle6.Y + 4, index44, rectangle6.Height - 8, (int) color4.R, (int) color4.G, (int) color4.B, (int) color4.A);
              trect2 = Rectangle::new(rectangle6.X + num83, rectangle6.Y + 4, index44, rectangle6.Height - 8);
              let mut trect6: &Rectangle = &trect2
              this.AddMouse(ref trect6, "", "Attacker has lost " + numArray4[index49].ToString() + " sub units.");
            }
            else
            {
              let mut num84: i32 =  4;
              let mut num85: i32 =  (int) Math.Round( (num76 * numArray4[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num84, rectangle6.Y + 4, num85, rectangle6.Height - 8, (int) color4.R, (int) color4.G, (int) color4.B, (int) color4.A);
              trect2 = Rectangle::new(rectangle6.X + num84, rectangle6.Y + 4, num85, rectangle6.Height - 8);
              let mut trect7: &Rectangle = &trect2
              this.AddMouse(ref trect7, "", "Defender has lost " + numArray4[index49].ToString() + " sub units.");
              let mut num86: i32 =  num84 + num85;
              let mut num87: i32 =  (int) Math.Round( (num76 * numArray5[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num86, rectangle6.Y + 4, num87, rectangle6.Height - 8, (int) color3.R, (int) color3.G, (int) color3.B, (int) color3.A);
              trect2 = Rectangle::new(rectangle6.X + num86, rectangle6.Y + 4, num87, rectangle6.Height - 8);
              let mut trect8: &Rectangle = &trect2
              this.AddMouse(ref trect8, "", "Defender has " + numArray5[index49].ToString() + " sub units that are neither fighting, nor retreating, nor killed.");
              let mut num88: i32 =  num86 + num87;
              let mut num89: i32 =  (int) Math.Round( (num76 * numArray2[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num88, rectangle6.Y + 4, num89, rectangle6.Height - 8, (int) color2.R, (int) color2.G, (int) color2.B, (int) color2.A);
              trect2 = Rectangle::new(rectangle6.X + num88, rectangle6.Y + 4, num89, rectangle6.Height - 8);
              let mut trect9: &Rectangle = &trect2
              this.AddMouse(ref trect9, "", "Defender has " + numArray2[index49].ToString() + " sub units that are retreating, or have retreated.");
              let mut num90: i32 =  num88 + num89;
              index44 = (int) Math.Round( (num76 * numArray3[index49]) /  numArray1[index49]);
              DrawMod.DrawBlock(ref graphics, rectangle6.X + num90, rectangle6.Y + 4, index44, rectangle6.Height - 8, (int) color1.R, (int) color1.G, (int) color1.B, (int) color1.A);
              trect2 = Rectangle::new(rectangle6.X + num90, rectangle6.Y + 4, index44, rectangle6.Height - 8);
              let mut trect10: &Rectangle = &trect2
              this.AddMouse(ref trect10, "", "Defender still has " + numArray3[index49].ToString() + " sub units fighting.");
            }
            DrawMod.DrawSimpleFrame(ref this.game.LINESFRAME, ref graphics, rectangle6.X, rectangle6.Y, rectangle6.Width, rectangle6.Height, 10, 10, 10);
            index45 = (int) Math.Round( (100 * numArray3[index49]) /  numArray1[index49]);
            str: String = index45.ToString() + "%";
            if (numArray6[index49] > 0)
              str += " ?";
            tstring: String = str + " Troops Operational";
            DrawMod.DrawTextColouredMarcCenter(ref graphics, tstring, this.game.MarcFont16, rectangle6.X + (int) Math.Round( rectangle6.Width / 2.0), rectangle6.Y + Math.Max(rectangle6.Height - 50, 0) + 6, Color.White);
          }
        }
        let mut num91: i32 =  rectangle4.Y + 15;
        if (index49 == 0)
        {
          index44 = this.game.TempCombat.DefenderRegime;
          num45 = rectangle4.X + rectangle4.Width - 100;
        }
        if (index49 == 1)
        {
          index44 = this.game.TempCombat.AttackerRegime;
          num45 = rectangle3.X + 13;
        }
        if (index44 == -1)
          index44 = 1;
        if (this.game.ScreenWidth < 1660)
        {
          let mut bannerSpriteNr: i32 =  this.game.Data.RegimeObj[index44].BannerSpriteNr;
          ref Graphics local29 = ref graphics;
          bitmap = BitmapStore.GetBitmap(bannerSpriteNr);
          ref local30: Bitmap = ref bitmap;
          let mut x7: i32 =  num45;
          let mut y5: i32 =  num91;
          double r1 =  ( red /  byte.MaxValue) - 1.0;
          double g1 =  ( green /  byte.MaxValue) - 1.0;
          double b1 =  ( blue /  byte.MaxValue) - 1.0;
          DrawMod.DrawScaledColorized(ref local29, ref local30, x7, y5, 80, 75, 124, 210,  r1,  g1,  b1, 1f);
          let mut bannerSpriteNr2: i32 =  this.game.Data.RegimeObj[index44].BannerSpriteNr2;
          if (bannerSpriteNr2 > 0)
          {
            ref Graphics local31 = ref graphics;
            bitmap = BitmapStore.GetBitmap(bannerSpriteNr2);
            ref local32: Bitmap = ref bitmap;
            let mut x8: i32 =  num45;
            let mut y6: i32 =  num91;
            double r2 =  ( red2 /  byte.MaxValue) - 1.0;
            double g2 =  ( green2 /  byte.MaxValue) - 1.0;
            double b2 =  ( blue2 /  byte.MaxValue) - 1.0;
            DrawMod.DrawScaledColorized(ref local31, ref local32, x8, y6, 80, 75, 124, 210,  r2,  g2,  b2, 1f);
          }
          let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[index44].HQSpriteNr2;
          if (hqSpriteNr2 > 0)
          {
            ref Graphics local33 = ref graphics;
            bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
            ref local34: Bitmap = ref bitmap;
            let mut x9: i32 =  num45 + 20;
            let mut y7: i32 =  num91 + 20;
            double r3 =  ( this.game.Data.RegimeObj[index44].Red3 /  byte.MaxValue) - 1.0;
            double g3 =  ( this.game.Data.RegimeObj[index44].Green3 /  byte.MaxValue) - 1.0;
            double b3 =  ( this.game.Data.RegimeObj[index44].Blue3 /  byte.MaxValue) - 1.0;
            DrawMod.Draw(ref local33, ref local34, x9, y7,  r3,  g3,  b3, 0.95f);
          }
        }
        else
        {
          let mut bannerSpriteNr: i32 =  this.game.Data.RegimeObj[index44].BannerSpriteNr;
          ref Graphics local35 = ref graphics;
          bitmap = BitmapStore.GetBitmap(bannerSpriteNr);
          ref local36: Bitmap = ref bitmap;
          let mut x10: i32 =  num45;
          let mut y8: i32 =  num91;
          double r4 =  ( red /  byte.MaxValue) - 1.0;
          double g4 =  ( green /  byte.MaxValue) - 1.0;
          double b4 =  ( blue /  byte.MaxValue) - 1.0;
          DrawMod.DrawScaledColorized(ref local35, ref local36, x10, y8, 80, 135, 124, 210,  r4,  g4,  b4, 1f);
          let mut bannerSpriteNr2: i32 =  this.game.Data.RegimeObj[index44].BannerSpriteNr2;
          if (bannerSpriteNr2 > 0)
          {
            ref Graphics local37 = ref graphics;
            bitmap = BitmapStore.GetBitmap(bannerSpriteNr2);
            ref local38: Bitmap = ref bitmap;
            let mut x11: i32 =  num45;
            let mut y9: i32 =  num91;
            double r5 =  ( red2 /  byte.MaxValue) - 1.0;
            double g5 =  ( green2 /  byte.MaxValue) - 1.0;
            double b5 =  ( blue2 /  byte.MaxValue) - 1.0;
            DrawMod.DrawScaledColorized(ref local37, ref local38, x11, y9, 80, 135, 124, 210,  r5,  g5,  b5, 1f);
          }
          let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[index44].HQSpriteNr2;
          if (hqSpriteNr2 > 0)
          {
            ref Graphics local39 = ref graphics;
            bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
            ref local40: Bitmap = ref bitmap;
            let mut x12: i32 =  num45 + 20;
            let mut y10: i32 =  num91 + 44;
            double r6 =  ( this.game.Data.RegimeObj[index44].Red3 /  byte.MaxValue) - 1.0;
            double g6 =  ( this.game.Data.RegimeObj[index44].Green3 /  byte.MaxValue) - 1.0;
            double b6 =  ( this.game.Data.RegimeObj[index44].Blue3 /  byte.MaxValue) - 1.0;
            DrawMod.Draw(ref local39, ref local40, x12, y10,  r6,  g6,  b6, 0.95f);
          }
        }
        if (index49 == 0)
          DrawMod.DrawTextColouredConsoleCenter(ref graphics, this.game.Data.RegimeObj[index44].Name, this.game.MarcFont16, num45 - 117, num91 + 5, this.game.seColWhite);
        if (index49 == 1)
          DrawMod.DrawTextColouredConsoleCenter(ref graphics, this.game.Data.RegimeObj[index44].Name, this.game.MarcFont16, num45 + 217, num91 + 5, this.game.seColWhite);
        index49 += 1;
      }
      while (index49 <= 1);
      Rectangle trect11;
      if (this.showdetail <= 0)
      {
        let mut num92: i32 =  0;
        do
        {
          let mut num93: i32 =  num3 - 1;
          for (let mut index50: i32 =  0; index50 <= num93; index50 += 1)
          {
            let mut num94: i32 =  num4 - 1;
            for (let mut index51: i32 =  0; index51 <= num94; index51 += 1)
            {
              num95: i32;
              index52: i32;
              if (num92 == 1)
              {
                num95 = this.crm[1, index51, index50 + this.attPage * num3, 0];
                index52 = this.crm[1, index51, index50 + this.attPage * num3, 1];
              }
              else
              {
                num95 = this.crm[0, index51, index50 + this.defPage * num3, 0];
                index52 = this.crm[0, index51, index50 + this.defPage * num3, 1];
              }
              color: Color = Color.White;
              colMod: Color = Color.White;
              str2: String = "";
              let mut nr9: i32 =  -1;
              switch (num95)
              {
                case 1:
                  color = this.GetUnitColor(index52);
                  color = Color.FromArgb((int) byte.MaxValue, 100, 100, 100);
                  break;
                case 2:
                  color = this.GetUnitColor(index52);
                  color = Color.FromArgb((int) byte.MaxValue, 100, 100, 100);
                  break;
                default:
                  if (num95 == 3 | num95 == 4)
                  {
                    index43 = this.game.TempCombat.FindISlot(index52);
                    colMod = Color.White;
                    if (this.game.TempCombat.IList[index43].ICapitulate)
                    {
                      color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 200, 200);
                      colMod = Color.White;
                      str2 = "capitulated".to_owned();
                      nr9 = this.game.SE1_COMBAT_SURRENDER;
                    }
                    else if (this.game.TempCombat.IList[index43].IKilled > 0)
                    {
                      color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 200, 200);
                      colMod = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 0, 0);
                      str2 = "dead".to_owned();
                      nr9 = this.game.SE1_COMBAT_DEAD;
                    }
                    else if (this.game.TempCombat.IList[index43].IRetreated == 0 & this.game.TempCombat.IList[index43].IRetreat > 0 & this.game.TempCombat.IList[index43].IRetreatMode == 1)
                    {
                      str2 = "forced retreat + in process of retreating";
                      nr9 = this.game.SE1_COMBAT_RETREATING;
                    }
                    else if (this.game.TempCombat.IList[index43].IRetreated == 0 & this.game.TempCombat.IList[index43].IRetreat > 0 & this.game.TempCombat.IList[index43].IRetreatMode == 3)
                    {
                      str2 = "panicked + in process of retreating";
                      nr9 = this.game.SE1_COMBAT_RETREATING;
                    }
                    else if (this.game.TempCombat.IList[index43].IRetreated == 0 & this.game.TempCombat.IList[index43].IRetreat > 0)
                    {
                      str2 = "orderly retreat - in process of retreating";
                      nr9 = this.game.SE1_COMBAT_RETREATING;
                    }
                    else if (this.game.TempCombat.IList[index43].IRetreated == 0 & this.game.TempCombat.IList[index43].IKilled == 0)
                      str2 = this.game.TempCombat.IList[index43].IBreakTrough <= 0 ? "fighting" : "fighting + broken through";
                    else if (this.game.TempCombat.IList[index43].IRetreated > 0)
                    {
                      if (this.game.TempCombat.IList[index43].IRetreatMode == 1 || this.game.TempCombat.IList[index43].IRetreatMode == 3)
                        ;
                      str2 = "safely retreated";
                      nr9 = this.game.SE1_COMBAT_RETREATED;
                    }
                    else
                    {
                      color = Color.White;
                      str2 = "fighting".to_owned();
                    }
                    if (num95 == 4 && this.game.TempCombat.IList[index43].IunitFeatStart > 0 && this.game.TempCombat.IList[index43].IunitFeatDeadRound > 0)
                    {
                      color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 200, 200);
                      colMod = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 0, 0);
                      str2 = "dead".to_owned();
                      nr9 = this.game.SE1_COMBAT_DEAD;
                      break;
                    }
                    break;
                  }
                  if (num95 != 5)
                    break;
                  break;
              }
              x13: i32;
              y11: i32;
              if (num92 == 1)
              {
                x13 = rectangle1.X + num2 * index51 + num5;
                y11 = rectangle1.Y + num2 * index50 + num6;
              }
              else
              {
                x13 = rectangle2.X + num2 * index51 + num5;
                y11 = rectangle2.Y + num2 * index50 + num6;
              }
              switch (num95)
              {
                case 0:
                  ref Graphics local41 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK3, this.useZoom);
                  ref local42: Bitmap = ref bitmap;
                  let mut x14: i32 =  x13;
                  let mut y12: i32 =  y11;
                  DrawMod.DrawSimple(ref local41, ref local42, x14, y12);
                  break;
                case 1:
                  ref Graphics local43 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK1B, this.useZoom);
                  ref local44: Bitmap = ref bitmap;
                  let mut x15: i32 =  x13;
                  let mut y13: i32 =  y11;
                  double r7 =  ( color.R /  byte.MaxValue) - 1.0;
                  double g7 =  ( color.G /  byte.MaxValue) - 1.0;
                  double b7 =  ( color.B /  byte.MaxValue) - 1.0;
                  double a1 =  ( color.A /  byte.MaxValue);
                  DrawMod.Draw(ref local43, ref local44, x15, y13,  r7,  g7,  b7,  a1);
                  ref Graphics local45 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK2B, this.useZoom);
                  ref local46: Bitmap = ref bitmap;
                  let mut x16: i32 =  x13;
                  let mut y14: i32 =  y11;
                  DrawMod.DrawSimple(ref local45, ref local46, x16, y14);
                  break;
                default:
                  if (num95 != 2 & num95 != 5)
                  {
                    color = Color.FromArgb((int) byte.MaxValue, (int) color.R + (int) Math.Round( ((int) byte.MaxValue - (int) color.R) * 0.66), (int) color.G + (int) Math.Round( ((int) byte.MaxValue - (int) color.G) * 0.66), (int) color.B + (int) Math.Round( ((int) byte.MaxValue - (int) color.B) * 0.66));
                    if (color.R == byte.MaxValue & color.G == byte.MaxValue & color.B == byte.MaxValue)
                    {
                      ref Graphics local47 = ref graphics;
                      bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK1, this.useZoom);
                      ref local48: Bitmap = ref bitmap;
                      let mut x17: i32 =  x13;
                      let mut y15: i32 =  y11;
                      DrawMod.DrawSimple(ref local47, ref local48, x17, y15);
                    }
                    else
                    {
                      ref Graphics local49 = ref graphics;
                      bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK1, this.useZoom);
                      ref local50: Bitmap = ref bitmap;
                      let mut x18: i32 =  x13;
                      let mut y16: i32 =  y11;
                      double r8 =  ( color.R /  byte.MaxValue) - 1.0;
                      double g8 =  ( color.G /  byte.MaxValue) - 1.0;
                      double b8 =  ( color.B /  byte.MaxValue) - 1.0;
                      double a2 =  ( color.A /  byte.MaxValue);
                      DrawMod.Draw(ref local49, ref local50, x18, y16,  r8,  g8,  b8,  a2);
                    }
                    ref Graphics local51 = ref graphics;
                    bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBLOCK2, this.useZoom);
                    ref local52: Bitmap = ref bitmap;
                    let mut x19: i32 =  x13;
                    let mut y17: i32 =  y11;
                    DrawMod.DrawSimple(ref local51, ref local52, x19, y17);
                    break;
                  }
                  break;
              }
              if (num95 == 1)
              {
                bool mirror = this.game.Data.RegimeObj[this.game.Data.UnitObj[index52].Regime].Mirror;
                if (this.game.TempCombat.AttackerRegime == this.game.Data.UnitObj[index52].Regime)
                {
                  if (mirror)
                    this.game.Data.RegimeObj[this.game.Data.UnitObj[index52].Regime].Mirror = false;
                }
                else if (!mirror)
                  this.game.Data.RegimeObj[this.game.Data.UnitObj[index52].Regime].Mirror = true;
                if (this.useZoom == 1)
                  this.game.CustomBitmapObj.DrawUnitBig(index52, toG: graphics, tx: (x13 + num7), ty: (y11 + num8), FullRecon: true);
                if (this.useZoom == 0)
                  this.game.CustomBitmapObj.DrawUnit(index52, toG: graphics, tx: (x13 + num7), ty: (y11 + num8), FullRecon: true, ForceHideUnitMode: 0);
                this.game.Data.RegimeObj[this.game.Data.UnitObj[index52].Regime].Mirror = mirror;
                name: String = this.game.Data.UnitObj[index52].Name;
                ttext: String = "[ " + this.GetUnitDescription(index52) + " ]";
                trect2 = Rectangle::new(x13, y11, num2 * 3, num2);
                trect11 = trect2;
                this.AddMouse(ref trect11, name, ttext);
              }
              SizeF sizeF2;
              if (num95 == 2)
              {
                if (this.useZoom == 1)
                {
                  str3: String = this.game.Data.UnitObj[index52].Name;
                  if (this.game.Data.UnitObj[index52].Regime != this.game.Data.Turn &&  this.game.Data.MapObj[0].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].MaxRecon <  this.game.Data.RuleVar[55])
                    str3 = "Unknown Unit";
                  sizeF2 = graphics.MeasureString(str3, this.game.MarcFont4, num2 * 2 - num7);
                  index43 = (int) Math.Round(( (int) Math.Round( num2 * 0.2) +  sizeF2.Height) / 2.0);
                  unitDescription: String = this.GetUnitDescription(index52);
                  if (unitDescription.Length < 2)
                    index43 = (int) Math.Round( (sizeF2.Height / 2f));
                  DrawMod.DrawTextColouredConsoleMultiline(ref graphics, str3, this.game.MarcFont4, x13 + num7, y11 - index43 + (int) Math.Round( num2 * 0.5), Color.LightGray, num2 * 2 - num7, num2);
                  DrawMod.DrawTextColouredMarc(ref graphics, unitDescription, this.game.MarcFont16, x13 + num7, (int) Math.Round( ( (y11 - index43 + (int) Math.Round( num2 * 0.5)) + sizeF2.Height)), Color.White);
                }
                if (this.useZoom == 0)
                {
                  lower: String = this.GetUnitDescription(index52).ToLower();
                  DrawMod.DrawTextColouredMarc(ref graphics, lower, this.game.MarcFont16, x13 + num7, y11 + (int) Math.Round( num2 * 0.5) - 9, Color.LightGray);
                }
              }
              if (num95 == 3)
              {
                str4: String = "";
                index43 = this.game.TempCombat.FindISlot(index52);
                if (!Information.IsNothing( this.game.TempCombat.IList[index43].IunitFeat))
                {
                  let mut counter12: i32 =  this.game.TempCombat.IList[index43].IunitFeat.Counter;
                  for (let mut index53: i32 =  0; index53 <= counter12; index53 += 1)
                  {
                    if (index53 <= 1)
                    {
                      let mut idValue: i32 =  this.game.TempCombat.IList[index43].IunitFeat.Id[index53];
                      if (idValue > 0)
                      {
                        let mut islot: i32 =  this.game.TempCombat.FindISlot(this.game.TempCombat.IList[index43].IunitFeat.Data1[index53]);
                        if (this.game.TempCombat.IList[islot].IunitFeatDeadRound < 1 & (this.game.TempCombat.IList[islot].IKilled < 1 | islot == index43))
                        {
                          let mut index54: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 10)));
                          str5: String = this.game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
                          if (str4.Length > 0 & index53 < this.game.TempCombat.IList[index43].IunitFeat.Counter)
                            str5 = ", " + str5;
                          if (str4.Length > 0 & index53 == this.game.TempCombat.IList[index43].IunitFeat.Counter)
                            str5 = " and " + str5;
                          str4 += str5;
                          let mut nr10: i32 =  this.game.Data.EventPicNr[index54];
                          num96: i32;
                          num97: i32;
                          num98: i32;
                          num99: i32;
                          if (this.useZoom == 1 & index53 == 0)
                          {
                            num96 = 2;
                            num97 = 2;
                            num98 = 40;
                            num99 = 40;
                          }
                          if (this.useZoom == 1 & index53 == 1)
                          {
                            num96 = 44;
                            num97 = 2;
                            num98 = 40;
                            num99 = 40;
                          }
                          if (this.useZoom == 0 & index53 == 0)
                          {
                            num96 = 0;
                            num97 = 0;
                            num98 = 20;
                            num99 = 20;
                          }
                          if (this.useZoom == 0 & index53 == 1)
                          {
                            num96 = 20;
                            num97 = 0;
                            num98 = 20;
                            num99 = 20;
                          }
                          BitmapStore.GetWidth(nr10);
                          BitmapStore.Getheight(nr10);
                          ref Graphics local53 = ref graphics;
                          bitmap = BitmapStore.GetBitmap(nr10);
                          ref local54: Bitmap = ref bitmap;
                          let mut x20: i32 =  x13 + num96;
                          let mut y18: i32 =  y11 + num97;
                          let mut w2: i32 =  num98;
                          let mut h: i32 =  num99;
                          let mut width: i32 =  BitmapStore.GetWidth(nr10);
                          let mut origh: i32 =  BitmapStore.Getheight(nr10);
                          double r9 =  ( colMod.R /  byte.MaxValue);
                          double g9 =  ( colMod.G /  byte.MaxValue);
                          double b9 =  ( colMod.B /  byte.MaxValue);
                          double a3 =  ( colMod.A /  byte.MaxValue);
                          DrawMod.DrawScaledColorized2(ref local53, ref local54, x20, y18, w2, h, width, origh,  r9,  g9,  b9,  a3);
                        }
                      }
                    }
                  }
                }
                this.DrawIndividual(graphics, x13 + num7, y11 + num8, index52, num9, num10, true, colMod, num92 == 0);
                if (nr9 > -1)
                {
                  bool flag3 = num92 == 1;
                  num100: i32;
                  if (this.useZoom == 1)
                    num100 = 76;
                  if (this.useZoom == 0)
                    num100 = 38;
                  if (flag3)
                  {
                    Matrix matrix2 = Matrix::new();
                    matrix2.Scale(-1f, 1f);
                    matrix2.Translate( -(2 * (x13 + num7) + num100), 0.0f);
                    graphics.Transform = matrix2;
                  }
                  ref Graphics local55 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(nr9, this.useZoom);
                  ref local56: Bitmap = ref bitmap;
                  let mut x21: i32 =  x13 + num7;
                  let mut y19: i32 =  y11 + num8;
                  DrawMod.DrawSimple(ref local55, ref local56, x21, y19);
                  if (flag3)
                    graphics.ResetTransform();
                }
                num101: i32;
                if (this.useZoom == 1)
                {
                  if (this.game.TempCombat.IList[index43].ItotalKills > 0 & this.game.TempCombat.IList[index43].ItotalHits > 0)
                  {
                    str6: String = this.game.TempCombat.IList[index43].ItotalKills.ToString();
                    str7: String = "/";
                    str8: String = this.game.TempCombat.IList[index43].ItotalHits.ToString();
                    sizeF2 = graphics.MeasureString(str6, this.game.MarcFont4, 76);
                    let mut num102: i32 =  (int) Math.Round( (sizeF2.Width - 2f));
                    sizeF2 = graphics.MeasureString(str7, this.game.MarcFont4, 76);
                    let mut num103: i32 =  (int) Math.Round( (sizeF2.Width - 2f));
                    sizeF2 = graphics.MeasureString(str8, this.game.MarcFont4, 76);
                    let mut num104: i32 =  (int) Math.Round( (sizeF2.Width - 2f));
                    if (num92 == 0)
                      num101 = 72 - (num102 + num103 + num104);
                    if (num92 == 1)
                      num101 = 8;
                    DrawMod.DrawTextColouredMarc(ref graphics, str6, this.game.MarcFont4, x13 + num7 + num101, y11 + 6, Color.Red);
                    DrawMod.DrawTextColouredMarc(ref graphics, str7, this.game.MarcFont4, x13 + num7 + num101 + num102, y11 + 6, Color.LightGray);
                    DrawMod.DrawTextColouredMarc(ref graphics, str8, this.game.MarcFont4, x13 + num7 + num101 + num102 + num103, y11 + 6, Color.White);
                  }
                  else if (this.game.TempCombat.IList[index43].ItotalKills > 0)
                  {
                    str9: String = this.game.TempCombat.IList[index43].ItotalKills.ToString();
                    sizeF2 = graphics.MeasureString(str9, this.game.MarcFont4, 76);
                    let mut num105: i32 =  (int) Math.Round( (sizeF2.Width - 2f));
                    if (num92 == 0)
                      num101 = 72 - num105;
                    if (num92 == 1)
                      num101 = 8;
                    DrawMod.DrawTextColouredMarc(ref graphics, str9, this.game.MarcFont4, x13 + num7 + num101, y11 + 6, Color.Red);
                  }
                  else if (this.game.TempCombat.IList[index43].ItotalHits > 0)
                  {
                    str10: String = this.game.TempCombat.IList[index43].ItotalHits.ToString();
                    sizeF2 = graphics.MeasureString(str10, this.game.MarcFont4, 76);
                    let mut num106: i32 =  (int) Math.Round( (sizeF2.Width - 2f));
                    if (num92 == 0)
                      num101 = 72 - num106;
                    if (num92 == 1)
                      num101 = 8;
                    DrawMod.DrawTextColouredMarc(ref graphics, str10, this.game.MarcFont4, x13 + num7 + num101, y11 + 6, Color.White);
                  }
                }
                if (this.useZoom == 1 && this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].DepletingHitpointRule > 0)
                {
                  let mut w3: i32 =  (int) Math.Round( ( (this.game.TempCombat.IList[index43].IHp * 56) /  this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].HitPoints[0]));
                  DrawMod.DrawBlock(ref graphics, x13 + num7 + 7, y11 + 8, 58, 6, 0, 100, 100, 155);
                  DrawMod.DrawBlock(ref graphics, x13 + 1 + num7 + 7, y11 + 9, w3, 4, 0, (int) byte.MaxValue, (int) byte.MaxValue, 155);
                }
                if (!(this.game.TempCombat.IList[index43].IKilled > 0 & this.game.TempCombat.IList[index43].IKilled < this.game.TempCombat.CombatRound) & this.game.TempCombat.CombatRound > 0)
                {
                  if (this.animList.FindNr(this.game.TempCombat.IList[index43].IID) == -1 & this.game.TempCombat.IList[index43].IKilled == this.game.TempCombat.CombatRound & !this.game.TempCombat.IList[index43].ICapitulate)
                    this.animList.Add(this.game.TempCombat.IList[index43].IID, 0, x13 + num7, y11 + num8, num9, num10, 1);
                  else if (this.animList.FindNr(this.game.TempCombat.IList[index43].IID) == -1 & this.game.TempCombat.IList[index43].ILastHit == this.game.TempCombat.CombatRound & this.game.TempCombat.IList[index43].IKilled < 1)
                    this.animList.Add(this.game.TempCombat.IList[index43].IID, 15, x13 + num7, y11 + num8, num9, num10, 2);
                }
                ttitle: String = this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].Ratio.ToString() + "x " + this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].Name;
                str11: String = "";
                if (str2.Length > 0)
                  str11 = str11 + " [ " + str2.ToUpper() + " ]\r\n---------------\r\n";
                ttext: String;
                if (this.game.TempCombat.IList[index43].IAttacker == 1)
                {
                  if (str4.Length > 0)
                    str11 = str11 + "Has embedded with them: " + str4 + ".\r\n" + "---------------" + "\r\n";
                  str12: String = str11;
                  index45 = (int) Math.Round( (this.game.TempCombat.IList[index43].ILisAmmoMod * 100f));
                  str13: String = index45.ToString();
                  str14: String = str12 + "Ammo modifier: \t" + str13 + "%\r\n";
                  index45 = (int) Math.Round( (this.game.TempCombat.IList[index43].ILisFuelMod * 100f));
                  str15: String = index45.ToString();
                  str16: String = str14 + "Fuel modifier: \t" + str15 + "%\r\n" + "---------------" + "\r\n" + "Readiness: \t" + this.game.TempCombat.IList[index43].IRdn.ToString() + " / " + this.game.TempCombat.IList[index43].IRdnInitial.ToString() + "\r\n" + "Morale: \t\t" + this.game.TempCombat.IList[index43].IMor.ToString() + " / " + this.game.TempCombat.IList[index43].IMorInitial.ToString() + "\r\n" + "Experience: \t" + this.game.TempCombat.IList[index43].IXp.ToString() + " / " + this.game.TempCombat.IList[index43].IXpInitial.ToString() + "\r\n" + "Entrenchment: \t" + this.game.TempCombat.IList[index43].IEntrench.ToString() + " / " + this.game.TempCombat.IList[index43].IEntrenchInitial.ToString() + "\r\n" + "Upkeep: \t\t" + this.game.Data.UnitObj[this.game.TempCombat.IList[index43].IUnr].SupplyConsume.ToString() + "\r\n" + "---------------\r\n";
                  if (this.game.TempCombat.IList[index43].IHp > 0)
                    str16 = str16 + "Shielding: \t" + this.game.TempCombat.IList[index43].IHp.ToString() + " / " + this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].HitPoints[0].ToString() + "\r\n" + "---------------\r\n";
                  str17: String = str16 + "Total Kills Scored: \t" + this.game.TempCombat.IList[index43].ItotalKills.ToString() + "\r\n" + "Total Hits Scored: \t" + this.game.TempCombat.IList[index43].ItotalHits.ToString() + "\r\n" + "---------------\r\n" + "Attacks done this Round: \t" + this.game.TempCombat.IList[index43].AttackCount.ToString() + "\r\n";
                  ttext = (this.game.TempCombat.IList[index43].ISuccesfullAttack != this.game.TempCombat.CombatRound ? str17 + "Hit placed this Round: \tNo\r\n" : str17 + "Hit placed this Round: \tYes\r\n") + "Been attacked this Round: \t" + this.game.TempCombat.IList[index43].AttackedCount.ToString() + "\r\n";
                  if (this.game.TempCombat.IList[index43].IRetreat > 0 & this.game.TempCombat.IList[index43].IKilled < 1)
                  {
                    str18: String = ttext + "---------------\r\n" + "Retreat started in Round: \t" + this.game.TempCombat.IList[index43].IRetreat.ToString() + "\r\n";
                    ttext = this.game.TempCombat.IList[index43].IRetreated <= 0 ? str18 + "Finished retreat: \t\tNot yet \r\n" : str18 + "Finished retreat: \t\tSafely retreated\r\n";
                  }
                }
                else
                {
                  if (str4.Length > 0)
                    str11 = str11 + "Has embedded with them: " + str4 + ".\r\n" + "---------------" + "\r\n";
                  str19: String = str11;
                  index45 = (int) Math.Round( (this.game.TempCombat.IList[index43].ILisAmmoMod * 100f));
                  str20: String = index45.ToString();
                  str21: String = str19 + "Ammo modifier: \t" + str20 + "%\r\n";
                  index45 = (int) Math.Round( (this.game.TempCombat.IList[index43].ILisFuelMod * 100f));
                  str22: String = index45.ToString();
                  str23: String = str21 + "Fuel modifier: \t" + str22 + "%\r\n" + "---------------" + "\r\n" + "Readiness: \t" + this.game.TempCombat.IList[index43].IRdn.ToString() + "\r\n" + "Morale: \t\t" + this.game.TempCombat.IList[index43].IMor.ToString() + "\r\n" + "Experience: \t" + this.game.TempCombat.IList[index43].IXp.ToString() + "\r\n" + "Entrenchment: \t" + this.game.TempCombat.IList[index43].IEntrench.ToString() + "\r\n" + "Upkeep: \t\t" + this.game.Data.UnitObj[this.game.TempCombat.IList[index43].IUnr].SupplyConsume.ToString() + "\r\n";
                  if (this.game.TempCombat.IList[index43].IHp > 0)
                    str23 = str23 + "Shielding: \t" + this.game.TempCombat.IList[index43].IHp.ToString() + " / " + this.game.Data.SFTypeObj[this.game.TempCombat.IList[index43].ISFType].HitPoints[0].ToString() + "\r\n" + "---------------\r\n";
                  str24: String = str23 + "---------------\r\n" + "Total Kills Scored: \t" + this.game.TempCombat.IList[index43].ItotalKills.ToString() + "\r\n" + "Total Hits Scored: \t" + this.game.TempCombat.IList[index43].ItotalHits.ToString() + "\r\n" + "---------------\r\n" + "Attacks done this Round: \t" + this.game.TempCombat.IList[index43].AttackCount.ToString() + "\r\n";
                  ttext = (this.game.TempCombat.IList[index43].ISuccesfullAttack != this.game.TempCombat.CombatRound ? str24 + "Hit placed this Round: \tNo\r\n" : str24 + "Hit placed this Round: \tYes\r\n") + "Been attacked this Round: \t" + this.game.TempCombat.IList[index43].AttackedCount.ToString() + "\r\n";
                  if (this.game.TempCombat.IList[index43].IRetreat > 0 & this.game.TempCombat.IList[index43].IKilled < 1)
                  {
                    str25: String = ttext + "---------------\r\n" + "Retreat started in Round: \t" + this.game.TempCombat.IList[index43].IRetreat.ToString() + "\r\n";
                    ttext = this.game.TempCombat.IList[index43].IRetreated <= 0 ? str25 + "Finished retreat: \t\tNot yet \r\n" : str25 + "Finished retreat: \t\tYes in Round " + this.game.TempCombat.IList[index43].IRetreated.ToString() + "\r\n";
                  }
                }
                trect2 = Rectangle::new(x13, y11, num2, num2);
                trect11 = trect2;
                this.AddMouse(ref trect11, ttitle, ttext);
              }
              if (num95 == 4)
              {
                index43 = this.game.TempCombat.FindISlot(index52);
                let mut iunitFeatStart: i32 =  this.game.TempCombat.IList[index43].IunitFeatStart;
                let mut index55: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotUnitFeats].GetData(0, iunitFeatStart, 10)));
                data: String = this.game.Data.StringListObj[this.slotUnitFeats].GetData(0, iunitFeatStart, 2);
                ttext: String = this.game.Data.StringListObj[this.slotUnitFeats].GetData(0, iunitFeatStart, 12);
                let mut nr11: i32 =  this.game.Data.EventPicNr[index55];
                num107: i32;
                num108: i32;
                num109: i32;
                num110: i32;
                if (this.useZoom == 1)
                {
                  num107 = 0;
                  num108 = 0;
                  num109 = 80;
                  num110 = 80;
                }
                if (this.useZoom == 0)
                {
                  num107 = 0;
                  num108 = 0;
                  num109 = 40;
                  num110 = 40;
                }
                let mut num111: i32 =  BitmapStore.GetWidth(nr11);
                let mut num112: i32 =  BitmapStore.Getheight(nr11);
                if (this.useZoom == 0)
                {
                  num111 = (int) Math.Round( num111 / 2.0);
                  num112 = (int) Math.Round( num112 / 2.0);
                }
                if (this.useZoom == 1)
                {
                  ref Graphics local57 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(nr11);
                  ref local58: Bitmap = ref bitmap;
                  let mut x22: i32 =  x13 + num107 + (int) Math.Round( (num109 - num111) / 2.0);
                  let mut y20: i32 =  y11 + num108 + (int) Math.Round( (num110 - num112) * 0.0) - 12;
                  double r10 =  ( colMod.R /  byte.MaxValue) - 1.0;
                  double g10 =  ( colMod.G /  byte.MaxValue) - 1.0;
                  double b10 =  ( colMod.B /  byte.MaxValue) - 1.0;
                  double a4 =  ( colMod.A /  byte.MaxValue);
                  DrawMod.Draw(ref local57, ref local58, x22, y20,  r10,  g10,  b10,  a4);
                }
                else if (this.useZoom == 0)
                {
                  ref Graphics local59 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(nr11);
                  ref local60: Bitmap = ref bitmap;
                  let mut x23: i32 =  x13 + num107 + (int) Math.Round( (num109 - num111) / 2.0);
                  let mut y21: i32 =  y11 + num108 + (int) Math.Round( (num110 - num112) * 0.0) - 4;
                  let mut w4: i32 =  num109;
                  let mut h: i32 =  num110;
                  double r11 =  ( colMod.R /  byte.MaxValue);
                  double g11 =  ( colMod.G /  byte.MaxValue);
                  double b11 =  ( colMod.B /  byte.MaxValue);
                  double a5 =  ( colMod.A /  byte.MaxValue);
                  DrawMod.DrawScaledColorized2(ref local59, ref local60, x23, y21, w4, h, 80, 80,  r11,  g11,  b11,  a5);
                }
                if (nr9 > -1)
                {
                  bool flag4 = num92 == 1;
                  if (this.useZoom == 1)
                    num109 = 76;
                  if (this.useZoom == 0)
                    num109 = 38;
                  if (flag4)
                  {
                    Matrix matrix3 = Matrix::new();
                    matrix3.Scale(-1f, 1f);
                    matrix3.Translate( -(2 * x13 + num109), 0.0f);
                    graphics.Transform = matrix3;
                  }
                  ref Graphics local61 = ref graphics;
                  bitmap = BitmapStore.GetBitmap(nr9, this.useZoom);
                  ref local62: Bitmap = ref bitmap;
                  let mut x24: i32 =  x13 + num7;
                  let mut y22: i32 =  y11 + num8;
                  DrawMod.DrawSimple(ref local61, ref local62, x24, y22);
                  if (flag4)
                    graphics.ResetTransform();
                }
                if (str2.Length > 0)
                  ttext = " [ " + str2.ToUpper() + " ]\r\n---------------\r\n" + ttext;
                trect2 = Rectangle::new(x13, y11, num2, num2);
                trect11 = trect2;
                this.AddMouse(ref trect11, data, ttext);
              }
            }
          }
          num92 += 1;
        }
        while (num92 <= 1);
      }
      ref Graphics local63 = ref graphics;
      bitmap = BitmapStore.GetBitmap(this.game.SE1_COMBATBAR2);
      ref local64: Bitmap = ref bitmap;
      let mut x25: i32 =  rectangle5.X;
      let mut y23: i32 =  rectangle5.Y;
      DrawMod.DrawSimple(ref local63, ref local64, x25, y23);
      combatDescription: String = this.GetCombatDescription();
      if (Strings.InStr(combatDescription, "\r\n") > 0)
        DrawMod.DrawTextColouredConsoleMultiline(ref graphics, combatDescription, this.game.MarcFont2, rectangle5.X, rectangle5.Y + 13, Color.White, rectangle5.Width, rectangle5.Height, true);
      else
        DrawMod.DrawTextColouredConsoleMultiline(ref graphics, combatDescription, this.game.MarcFont2, rectangle5.X, rectangle5.Y + 30, Color.White, rectangle5.Width, rectangle5.Height, true);
      let mut x26: i32 =  rectangle5.X + rectangle5.Width + 13;
      let mut y24: i32 =  rectangle5.Y + 50;
      bool flag5 = false;
      if (x26 + BitmapStore.GetWidth(this.game.SE1_SIDEBARHEADER) > rectangle4.X)
      {
        x26 = rectangle4.X - BitmapStore.GetWidth(this.game.SE1_SIDEBARHEADER);
        y24 += 10;
        flag5 = true;
      }
      ref Graphics local65 = ref graphics;
      bitmap = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
      ref local66: Bitmap = ref bitmap;
      let mut x27: i32 =  x26;
      let mut y25: i32 =  y24;
      DrawMod.DrawSimple(ref local65, ref local66, x27, y25);
      let mut num113: i32 =  this.game.Data.MapObj[0].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].MaxRecon;
      if (num113 > (int) Math.Round( this.game.Data.RuleVar[56]))
        num113 = (int) Math.Round( this.game.Data.RuleVar[56]);
      str26: String = "Rec: " + num113.ToString();
      index45 = (int) Math.Round( this.game.Data.RuleVar[56]);
      str27: String = index45.ToString();
      tstring3: String = str26 + " / " + str27;
      DrawMod.DrawTextColouredConsoleCenter(ref graphics, tstring3, this.game.MarcFont4, x26 + 70, y24 + 11, Color.LightGray);
      ttitle1: String = "Recon on target Hex";
      index45 = (int) Math.Round( this.game.Data.RuleVar[56]);
      ttext1: String = "As long as you have less than " + index45.ToString() + " Recon Points on the Hex you are attacking you'll not be sure to spot all enemy sub formations.";
      trect2 = Rectangle::new(x26, y24, 137, 39);
      trect11 = trect2;
      this.AddMouse(ref trect11, ttitle1, ttext1);
      if (this.showdetail < 1)
      {
        str28: String = "";
        if (this.game.TempCombat.BattleEnded > 0)
        {
          str29: String = this.game.EditObj.CombatOneSentence;
          if (this.game.EditObj.CombatOneSentenceCustom.Length > 0 & str29.Length > 0)
            str29 = str29 + "\r\n•" + this.game.EditObj.CombatOneSentenceCustom;
          else if (this.game.EditObj.CombatOneSentenceCustom.Length > 0)
            str29 = "• " + this.game.EditObj.CombatOneSentenceCustom;
          if (str29.Length > 1)
          {
            str30: String = str29.Replace(".", ".\r\n•");
            if (Operators.CompareString(Strings.Right(str30, 1), "•", false) == 0)
              str30 = Strings.Left(str30, Strings.Len(str30) - 2);
            if (Operators.CompareString(Strings.Right(str30, 2), "• ", false) == 0)
              str30 = Strings.Left(str30, Strings.Len(str30) - 3);
            str28 = str28 + str30 + "\r\n";
          }
          if (str28.Length > 1)
          {
            let mut num114: i32 =  0;
            let mut length: i32 =  str28.Length;
            for (let mut Start: i32 =  1; Start <= length; Start += 1)
            {
              if (Operators.CompareString(Strings.Mid(str28, Start, 1), ".", false) == 0)
                num114 += 1;
            }
            if (num114 < 1)
              num114 = 1;
            let mut x28: i32 =  rectangle5.X - 155;
            let mut y26: i32 =  49;
            if (flag5)
            {
              x28 = rectangle3.X + rectangle3.Width - 6;
              y26 += 10;
            }
            ref Graphics local67 = ref graphics;
            bitmap = BitmapStore.GetBitmap(this.game.SE1_SIDEBARHEADER);
            ref local68: Bitmap = ref bitmap;
            let mut x29: i32 =  x28;
            let mut y27: i32 =  y26;
            DrawMod.DrawSimple(ref local67, ref local68, x29, y27);
            tstring4: String = num114.ToString() + " Message(s)";
            DrawMod.DrawTextColouredConsoleCenter(ref graphics, tstring4, this.game.MarcFont4, x28 + 70, y26 + 12, Color.White);
            ttitle2: String = "MESSAGES".to_owned();
            trect2 = Rectangle::new(x28, y26, 137, 39);
            trect11 = trect2;
            this.AddMouse(ref trect11, ttitle2, str28);
          }
        }
      }
      this.Hn = this.useHeight;
      if (this.useZoom == 1)
      {
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("ZOOM IN", 90, "Click to use big icons.", ref this.OwnBitmap, 50, this.Hn - 51, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.zoom1id = this.AddSubPart(ref tsubpart1, 50, this.Hn - 51, 90, 36, 0);
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("ZOOM OUT", 90, "Click to use small icons.", ref this.OwnBitmap, 150, this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.zoom0id = this.AddSubPart(ref tsubpart2, 150, this.Hn - 51, 90, 36, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("ZOOM IN", 90, "Click to use big icons.", ref this.OwnBitmap, 50, this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.zoom1id = this.AddSubPart(ref tsubpart3, 50, this.Hn - 51, 90, 36, 1);
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("ZOOM OUT", 90, "Click to use small icons.", ref this.OwnBitmap, 150, this.Hn - 51, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.zoom0id = this.AddSubPart(ref tsubpart4, 150, this.Hn - 51, 90, 36, 0);
      }
      let mut num115: i32 =  250;
      num116: i32;
      SubPartClass tsubpart5;
      if (this.game.TempCombat.BattleEnded > 0)
      {
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("OK", 200, "Click to return to main screen.\r\nOr press any key instead.", ref this.OwnBitmap, (int) Math.Round( this.useWidth / 2.0 - 104.0), this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart(ref tsubpart6, (int) Math.Round( this.useWidth / 2.0 - 104.0), this.Hn - 51, 200, 36, 1);
        num116 = (int) Math.Round( this.useWidth / 2.0 - 104.0 + 200.0 + 10.0);
      }
      else
      {
        if (!this.playBattle & !this.game.EditObj.AutoCombat)
        {
          let mut tsubpart7: SubPartClass =  new TextButtonPartClass("NEXT ROUND", 150, "Click to initiate next Combat Round.\r\nOr press any key instead.", ref this.OwnBitmap, (int) Math.Round( this.useWidth / 2.0 - 154.0), this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          this.roundId = this.AddSubPart(ref tsubpart7, (int) Math.Round( this.useWidth / 2.0 - 154.0), this.Hn - 51, 150, 36, 1);
          tsubpart5 =  new TextButtonPartClass("AUTOPLAY", 150, "Click to initiate next Combat Round.\r\nOr press any key instead.", ref this.OwnBitmap, (int) Math.Round( this.useWidth / 2.0 - 4.0), this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          this.playId = this.AddSubPart(ref tsubpart5, (int) Math.Round( this.useWidth / 2.0 - 4.0), this.Hn - 51, 150, 36, 1);
        }
        num116 = (int) Math.Round( this.useWidth / 2.0 - 154.0 + 300.0 + 10.0);
      }
      tsubpart5 =  new TextButtonPartClass("DETAIL", 90, "Click to get deepest detail level.\r\nOr press any key instead.", ref this.OwnBitmap, this.useWidth - 146, this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.detailid = this.AddSubPart(ref tsubpart5, this.useWidth - 146, this.Hn - 51, 90, 36, 1);
      if (this.showdetail <= 0)
      {
        tsubpart5 =  new TextButtonPartClass("TEXTUAL", 90, "Click to switch to a numerical combat overview.", ref this.OwnBitmap, this.useWidth - 256, this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.typeid = this.AddSubPart(ref tsubpart5, this.useWidth - 256, this.Hn - 51, 90, 36, 1);
      }
      else
      {
        tsubpart5 =  new TextButtonPartClass("GRAPHIC", 90, "Click to switch to a graphical combat overview.", ref this.OwnBitmap, this.useWidth - 256, this.Hn - 51, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.typeid = this.AddSubPart(ref tsubpart5, this.useWidth - 256, this.Hn - 51, 90, 36, 1);
      }
      index56: i32;
      if (this.maxAttPage > 3)
      {
        let mut num117: i32 =  num115 + 30;
        let mut num118: i32 =  this.Hn - 51;
        if (this.attPage > 0)
        {
          int[] tabdown = this.tabdown;
          tsubpart5 =  new TextButtonPartClass("< ", 50, "Click to go back a page.", ref this.OwnBitmap, num117, num118, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num119: i32 =  this.AddSubPart(ref tsubpart5, num117, num118, 50, 36, 1);
          tabdown[1] = num119;
        }
        else
        {
          int[] tabdown = this.tabdown;
          tsubpart5 =  new TextButtonPartClass("< ", 50, "You are at the first page.", ref this.OwnBitmap, num117, num118, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num120: i32 =  this.AddSubPart(ref tsubpart5, num117, num118, 50, 36, 0);
          tabdown[1] = num120;
        }
        let mut num121: i32 =  num117 + 50;
        index56 = this.attPage + 1;
        str31: String = index56.ToString();
        index45 = this.maxAttPage + 1;
        str32: String = index45.ToString();
        tstring5: String = "Page " + str31 + " of " + str32;
        DrawMod.DrawTextColouredConsoleCenter(ref graphics, tstring5, this.game.MarcFont4, num121 + 75, num118 + 7, Color.LightGray);
        let mut num122: i32 =  num121 + 150;
        if (this.attPage < this.maxAttPage)
        {
          int[] tabup = this.tabup;
          tsubpart5 =  new TextButtonPartClass("> ", 50, "Click to go forward a page.", ref this.OwnBitmap, num122, num118, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num123: i32 =  this.AddSubPart(ref tsubpart5, num122, num118, 50, 36, 1);
          tabup[1] = num123;
        }
        else
        {
          int[] tabup = this.tabup;
          tsubpart5 =  new TextButtonPartClass("> ", 50, "You are at the last page.", ref this.OwnBitmap, num122, num118, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num124: i32 =  this.AddSubPart(ref tsubpart5, num122, num118, 50, 36, 0);
          tabup[1] = num124;
        }
      }
      else if (this.maxAttPage > 0)
      {
        let mut num125: i32 =  num115;
        let mut num126: i32 =  this.Hn - 51;
        let mut maxAttPage: i32 =  this.maxAttPage;
        for (let mut index57: i32 =  0; index57 <= maxAttPage; index57 += 1)
        {
          if (index57 < 9)
          {
            if (index57 != this.attPage)
            {
              tabid: Vec<i32> = this.tabid;
              let mut index58: i32 =  index57;
              index56 = index57 + 1;
              tsubpart5 =  new TextButtonPartClass("Pg " + index56.ToString(), 50, "Click to see this page.", ref this.OwnBitmap, num125, num126, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num127: i32 =  this.AddSubPart(ref tsubpart5, num125, num126, 50, 36, 1);
              tabid[1, index58] = num127;
            }
            else
            {
              tabid: Vec<i32> = this.tabid;
              let mut index59: i32 =  index57;
              index56 = index57 + 1;
              tsubpart5 =  new TextButtonPartClass("Pg " + index56.ToString(), 50, "This page is currently selected.", ref this.OwnBitmap, num125, num126, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num128: i32 =  this.AddSubPart(ref tsubpart5, num125, num126, 50, 36, 0);
              tabid[1, index59] = num128;
            }
            num125 += 50;
          }
        }
      }
      if (this.maxDefPage > 3)
      {
        let mut num129: i32 =  num116 + 30;
        let mut num130: i32 =  this.Hn - 51;
        if (this.defPage > 0)
        {
          int[] tabdown = this.tabdown;
          tsubpart5 =  new TextButtonPartClass("< ", 50, "Click to go back a page.", ref this.OwnBitmap, num129, num130, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num131: i32 =  this.AddSubPart(ref tsubpart5, num129, num130, 50, 36, 1);
          tabdown[0] = num131;
        }
        else
        {
          int[] tabdown = this.tabdown;
          tsubpart5 =  new TextButtonPartClass("< ", 50, "You are at the first page.", ref this.OwnBitmap, num129, num130, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num132: i32 =  this.AddSubPart(ref tsubpart5, num129, num130, 50, 36, 0);
          tabdown[0] = num132;
        }
        let mut num133: i32 =  num129 + 50;
        index45 = this.defPage + 1;
        str33: String = index45.ToString();
        index56 = this.maxDefPage + 1;
        str34: String = index56.ToString();
        tstring6: String = "Page " + str33 + " of " + str34;
        DrawMod.DrawTextColouredConsoleCenter(ref graphics, tstring6, this.game.MarcFont4, num133 + 75, num130 + 7, Color.LightGray);
        let mut num134: i32 =  num133 + 150;
        if (this.defPage < this.maxDefPage)
        {
          int[] tabup = this.tabup;
          tsubpart5 =  new TextButtonPartClass("> ", 50, "Click to go forward a page.", ref this.OwnBitmap, num134, num130, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num135: i32 =  this.AddSubPart(ref tsubpart5, num134, num130, 50, 36, 1);
          tabup[0] = num135;
        }
        else
        {
          int[] tabup = this.tabup;
          tsubpart5 =  new TextButtonPartClass("> ", 50, "You are at the last page.", ref this.OwnBitmap, num134, num130, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
          let mut num136: i32 =  this.AddSubPart(ref tsubpart5, num134, num130, 50, 36, 0);
          tabup[0] = num136;
        }
      }
      else if (this.maxDefPage > 0)
      {
        let mut num137: i32 =  num116;
        let mut num138: i32 =  this.Hn - 51;
        let mut maxDefPage: i32 =  this.maxDefPage;
        for (let mut index60: i32 =  0; index60 <= maxDefPage; index60 += 1)
        {
          if (index60 < 9)
          {
            if (index60 != this.defPage)
            {
              tabid: Vec<i32> = this.tabid;
              let mut index61: i32 =  index60;
              index56 = index60 + 1;
              tsubpart5 =  new TextButtonPartClass("Pg " + index56.ToString(), 50, "Click to see this page.", ref this.OwnBitmap, num137, num138, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num139: i32 =  this.AddSubPart(ref tsubpart5, num137, num138, 50, 36, 1);
              tabid[0, index61] = num139;
            }
            else
            {
              tabid: Vec<i32> = this.tabid;
              let mut index62: i32 =  index60;
              index56 = index60 + 1;
              tsubpart5 =  new TextButtonPartClass("Pg " + index56.ToString(), 50, "This page is currently selected.", ref this.OwnBitmap, num137, num138, true, theight: 36, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num140: i32 =  this.AddSubPart(ref tsubpart5, num137, num138, 50, 36, 0);
              tabid[0, index62] = num140;
            }
            num137 += 50;
          }
        }
      }
      object[,] objArray1 = new object[2, 100];
      object[,] objArray2 = new object[2, 100];
      int[,,] numArray19 = new int[2, 100, 9];
      object[,,,] objArray3 = new object[2, 100, 9, 3];
      let mut index63: i32 =  -1;
      let mut index64: i32 =  -1;
      let mut ucounter: i32 =  this.game.TempCombat.UCounter;
      for (let mut index65: i32 =  0; index65 <= ucounter; index65 += 1)
      {
        if (this.game.TempCombat.UList[index65].Uattacker == 1)
        {
          index63 += 1;
          objArray1[1, index63] =  this.game.TempCombat.UList[index65].UNr;
          objArray2[1, index63] =  index65;
          let mut unr: i32 =  this.game.TempCombat.UList[index65].UNr;
          let mut index66: i32 =  0;
          do
          {
            numArray19[1, index63, index66] = -1;
            index66 += 1;
          }
          while (index66 <= 7);
          let mut num141: i32 =  Math.Min(7, this.game.Data.UnitObj[unr].SFCount);
          for (let mut index67: i32 =  0; index67 <= num141; index67 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index67];
            numArray19[1, index63, index67] = sf;
          }
        }
        else
        {
          index64 += 1;
          objArray1[0, index64] =  this.game.TempCombat.UList[index65].UNr;
          objArray2[0, index64] =  index65;
          let mut unr: i32 =  this.game.TempCombat.UList[index65].UNr;
          let mut index68: i32 =  0;
          do
          {
            numArray19[0, index64, index68] = -1;
            index68 += 1;
          }
          while (index68 <= 7);
          let mut num142: i32 =  Math.Min(7, this.game.Data.UnitObj[unr].SFCount);
          for (let mut index69: i32 =  0; index69 <= num142; index69 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index69];
            numArray19[0, index64, index69] = sf;
          }
        }
      }
      let mut icounter3: i32 =  this.game.TempCombat.ICounter;
      for (let mut index70: i32 =  0; index70 <= icounter3; index70 += 1)
      {
        let mut num143: i32 =  !this.game.TempCombat.IList[index70].ICapitulate ? (!(this.game.TempCombat.IList[index70].IRetreated == 0 & this.game.TempCombat.IList[index70].IKilled == 0) ? (!((this.game.TempCombat.IList[index70].IRetreated > 0 | this.game.TempCombat.IList[index70].IRetreat > 0 & this.game.TempCombat.IList[index70].IRetreatMode != 2) & this.game.TempCombat.IList[index70].IKilled == 0) ? (this.game.TempCombat.IList[index70].IKilled <= 0 ? 1 : 2) : 1) : 0) : 2;
        let mut iattacker: i32 =  this.game.TempCombat.IList[index70].IAttacker;
        if (iattacker == 0)
          index43 = index64;
        if (iattacker == 1)
          index43 = index63;
        let mut num144: i32 =  index43;
        for (let mut index71: i32 =  0; index71 <= num144; index71 += 1)
        {
          if (Operators.ConditionalCompareObjectEqual(objArray1[iattacker, index71],  this.game.TempCombat.IList[index70].IUnr, false))
          {
            let mut index72: i32 =  0;
            do
            {
              if (numArray19[iattacker, index71, index72] == this.game.TempCombat.IList[index70].ISFNr)
              {
                object[,,,] objArray4 = objArray3;
                object[,,,] objArray5 = objArray4;
                index56 = iattacker;
                let mut index73: i32 =  index56;
                index45 = index71;
                let mut index74: i32 =  index45;
                let mut index75: i32 =  index72;
                let mut index76: i32 =  index75;
                let mut index77: i32 =  num143;
                let mut index78: i32 =  index77;
                object obj = Operators.AddObject(objArray4[index56, index45, index75, index77],  1);
                objArray5[index73, index74, index76, index78] = obj;
              }
              index72 += 1;
            }
            while (index72 <= 7);
          }
        }
      }
      numArray20: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray21: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      numArray22: Vec<i32> = new int[this.game.Data.SFTypeCounter + 1, 2];
      int[] numArray23 = new int[2];
      int[] numArray24 = new int[2];
      int[] numArray25 = new int[2];
      int[] numArray26 = new int[2];
      int[] numArray27 = new int[2];
      int[] numArray28 = new int[2];
      int[] numArray29 = new int[2];
      int[] numArray30 = new int[2];
      int[] numArray31 = new int[2];
      let mut num145: i32 =  0;
      let mut num146: i32 =  0;
      numArray23[0] = 0;
      numArray23[1] = 0;
      numArray24[0] = 0;
      numArray24[1] = 0;
      numArray25[0] = 0;
      numArray25[1] = 0;
      numArray26[0] = 0;
      numArray26[1] = 0;
      numArray27[0] = 0;
      numArray27[1] = 0;
      numArray28[0] = 0;
      numArray28[1] = 0;
      numArray29[0] = 0;
      numArray29[1] = 0;
      numArray30[0] = 0;
      numArray30[1] = 0;
      numArray30[0] = 0;
      this.StartRdn[0] = 0;
      this.StartXp[0] = 0;
      this.StartMor[0] = 0;
      this.StartEntr[0] = 0;
      numArray30[1] = 0;
      this.StartRdn[1] = 0;
      this.StartXp[1] = 0;
      this.StartMor[1] = 0;
      this.StartEntr[1] = 0;
      let mut num147: i32 =  0;
      num56 = 0;
      let mut icounter4: i32 =  this.game.TempCombat.ICounter;
      for (let mut index79: i32 =  0; index79 <= icounter4; index79 += 1)
      {
        let mut iattacker: i32 =  this.game.TempCombat.IList[index79].IAttacker;
        let mut isfType: i32 =  this.game.TempCombat.IList[index79].ISFType;
        let mut isfNr: i32 =  this.game.TempCombat.IList[index79].ISFNr;
        bool flag6 = false;
        if ( this.game.Data.MapObj[0].HexObj[this.game.TempCombat.TargetX, this.game.TempCombat.TargetY].MaxRecon >=  this.game.TempCombat.IList[index79].IcoverPoints)
          flag6 = true;
        if (!this.game.Data.FOWOn)
          flag6 = true;
        if (iattacker == 1)
          flag6 = true;
        if (flag6)
        {
          int[] numArray32 = numArray31;
          int[] numArray33 = numArray32;
          let mut index80: i32 =  iattacker;
          let mut index81: i32 =  index80;
          let mut num148: i32 =  numArray32[index80] + 1;
          numArray33[index81] = num148;
          int[] numArray34 = numArray30;
          int[] numArray35 = numArray34;
          let mut index82: i32 =  iattacker;
          let mut index83: i32 =  index82;
          let mut num149: i32 =  numArray34[index82] + this.game.Data.SFTypeObj[isfType].PowerPts;
          numArray35[index83] = num149;
          int[] startRdn = this.StartRdn;
          int[] numArray36 = startRdn;
          let mut index84: i32 =  iattacker;
          let mut index85: i32 =  index84;
          let mut num150: i32 =  startRdn[index84] + this.game.Data.SFObj[isfNr].Rdn;
          numArray36[index85] = num150;
          int[] startXp = this.StartXp;
          int[] numArray37 = startXp;
          let mut index86: i32 =  iattacker;
          let mut index87: i32 =  index86;
          let mut num151: i32 =  startXp[index86] + this.game.Data.SFObj[isfNr].Xp;
          numArray37[index87] = num151;
          int[] startMor = this.StartMor;
          int[] numArray38 = startMor;
          let mut index88: i32 =  iattacker;
          let mut index89: i32 =  index88;
          let mut num152: i32 =  startMor[index88] + this.game.Data.SFObj[isfNr].Mor;
          numArray38[index89] = num152;
          int[] startEntr = this.StartEntr;
          int[] numArray39 = startEntr;
          let mut index90: i32 =  iattacker;
          let mut index91: i32 =  index90;
          let mut num153: i32 =  startEntr[index90] + this.game.Data.SFObj[isfNr].CurrentEntrench;
          numArray39[index91] = num153;
          if (iattacker == 1)
            num147 += 1;
          else
            num56 += 1;
          if (this.game.TempCombat.IList[index79].IKilled > 0)
          {
            numArray40: Vec<i32> = numArray20;
            numArray41: Vec<i32> = numArray40;
            let mut index92: i32 =  isfType;
            let mut index93: i32 =  index92;
            let mut index94: i32 =  iattacker;
            let mut index95: i32 =  index94;
            let mut num154: i32 =  numArray40[index92, index94] + 1;
            numArray41[index93, index95] = num154;
            int[] numArray42 = numArray27;
            int[] numArray43 = numArray42;
            let mut index96: i32 =  iattacker;
            let mut index97: i32 =  index96;
            let mut num155: i32 =  numArray42[index96] + this.game.Data.SFTypeObj[isfType].PowerPts;
            numArray43[index97] = num155;
          }
          else if (this.game.TempCombat.IList[index79].IRetreat > 0)
          {
            numArray44: Vec<i32> = numArray21;
            numArray45: Vec<i32> = numArray44;
            let mut index98: i32 =  isfType;
            let mut index99: i32 =  index98;
            let mut index100: i32 =  iattacker;
            let mut index101: i32 =  index100;
            let mut num156: i32 =  numArray44[index98, index100] + 1;
            numArray45[index99, index101] = num156;
            int[] numArray46 = numArray28;
            int[] numArray47 = numArray46;
            let mut index102: i32 =  iattacker;
            let mut index103: i32 =  index102;
            let mut num157: i32 =  numArray46[index102] + this.game.Data.SFTypeObj[isfType].PowerPts;
            numArray47[index103] = num157;
          }
          else
          {
            numArray48: Vec<i32> = numArray22;
            numArray49: Vec<i32> = numArray48;
            let mut index104: i32 =  isfType;
            let mut index105: i32 =  index104;
            let mut index106: i32 =  iattacker;
            let mut index107: i32 =  index106;
            let mut num158: i32 =  numArray48[index104, index106] + 1;
            numArray49[index105, index107] = num158;
            int[] numArray50 = numArray29;
            int[] numArray51 = numArray50;
            let mut index108: i32 =  iattacker;
            let mut index109: i32 =  index108;
            let mut num159: i32 =  numArray50[index108] + this.game.Data.SFTypeObj[isfType].PowerPts;
            numArray51[index109] = num159;
          }
          if (this.game.TempCombat.IList[index79].IKilled <= 0)
          {
            if (iattacker == 1)
              num145 += 1;
            else
              num146 += 1;
            int[] numArray52 = numArray23;
            int[] numArray53 = numArray52;
            let mut index110: i32 =  iattacker;
            let mut index111: i32 =  index110;
            let mut num160: i32 =  numArray52[index110] + this.game.TempCombat.IList[index79].IRdn;
            numArray53[index111] = num160;
            int[] numArray54 = numArray24;
            int[] numArray55 = numArray54;
            let mut index112: i32 =  iattacker;
            let mut index113: i32 =  index112;
            let mut num161: i32 =  numArray54[index112] + this.game.TempCombat.IList[index79].IXp;
            numArray55[index113] = num161;
            int[] numArray56 = numArray25;
            int[] numArray57 = numArray56;
            let mut index114: i32 =  iattacker;
            let mut index115: i32 =  index114;
            let mut num162: i32 =  numArray56[index114] + this.game.TempCombat.IList[index79].IMor;
            numArray57[index115] = num162;
            int[] numArray58 = numArray26;
            int[] numArray59 = numArray58;
            let mut index116: i32 =  iattacker;
            let mut index117: i32 =  index116;
            let mut num163: i32 =  numArray58[index116] + this.game.TempCombat.IList[index79].IEntrench;
            numArray59[index117] = num163;
          }
        }
      }
      if (num145 < 1)
        num145 = 1;
      if (num146 < 1)
        num146 = 1;
      if (this.game.TempCombat.CombatRound == 0)
      {
        if (num56 > 0)
        {
          this.StartRdn[0] = (int) Math.Round(Conversion.Int( this.StartRdn[0] /  num56));
          this.StartXp[0] = (int) Math.Round(Conversion.Int( this.StartXp[0] /  num56));
          this.StartMor[0] = (int) Math.Round(Conversion.Int( this.StartMor[0] /  num56));
          this.StartEntr[0] = (int) Math.Round(Conversion.Int( this.StartEntr[0] /  num56));
        }
        else
        {
          this.StartRdn[0] = -1;
          this.StartXp[0] = -1;
          this.StartMor[0] = -1;
          this.StartEntr[0] = -1;
        }
        this.StartRdn[1] = (int) Math.Round(Conversion.Int( this.StartRdn[1] /  num147));
        this.StartXp[1] = (int) Math.Round(Conversion.Int( this.StartXp[1] /  num147));
        this.StartMor[1] = (int) Math.Round(Conversion.Int( this.StartMor[1] /  num147));
        this.StartEntr[1] = (int) Math.Round(Conversion.Int( this.StartEntr[1] /  num147));
        let mut index118: i32 =  0;
        do
        {
          this.game.EditObj.StartRdn[index118] = this.StartRdn[index118];
          this.game.EditObj.StartXp[index118] = this.StartXp[index118];
          this.game.EditObj.StartMor[index118] = this.StartMor[index118];
          this.game.EditObj.StartEntr[index118] = this.StartEntr[index118];
          index118 += 1;
        }
        while (index118 <= 1);
      }
      if (num146 > 0)
      {
        numArray23[0] = (int) Math.Round(Conversion.Int( numArray23[0] /  num146));
        numArray24[0] = (int) Math.Round(Conversion.Int( numArray24[0] /  num146));
        numArray25[0] = (int) Math.Round(Conversion.Int( numArray25[0] /  num146));
        numArray26[0] = (int) Math.Round(Conversion.Int( numArray26[0] /  num146));
      }
      else
      {
        numArray23[0] = -1;
        numArray24[0] = -1;
        numArray25[0] = -1;
        numArray26[0] = -1;
      }
      numArray23[1] = (int) Math.Round(Conversion.Int( numArray23[1] /  num145));
      numArray24[1] = (int) Math.Round(Conversion.Int( numArray24[1] /  num145));
      numArray25[1] = (int) Math.Round(Conversion.Int( numArray25[1] /  num145));
      numArray26[1] = (int) Math.Round(Conversion.Int( numArray26[1] /  num145));
      let mut index119: i32 =  0;
      do
      {
        this.StartRdn[index119] = this.game.EditObj.StartRdn[index119];
        this.StartXp[index119] = this.game.EditObj.StartXp[index119];
        this.StartMor[index119] = this.game.EditObj.StartMor[index119];
        this.StartEntr[index119] = this.game.EditObj.StartEntr[index119];
        index119 += 1;
      }
      while (index119 <= 1);
      if (this.showdetail == 1)
      {
        str35: String = "ATTACKER TOTALS\r\n";
        Expression1: String = "SUBFORMATIONTYPE".to_owned();
        let mut Number1: i32 =  35 - Strings.Len(Expression1);
        if (0 > Number1)
          Number1 = 0;
        str36: String = str35 + Expression1 + Strings.Space(Number1);
        Expression2: String = "INITIAL".to_owned();
        let mut Number2: i32 =  10 - Strings.Len(Expression2);
        if (0 > Number2)
          Number2 = 0;
        str37: String = str36 + Expression2 + Strings.Space(Number2);
        Expression3: String = "@FRONT";
        let mut Number3: i32 =  9 - Strings.Len(Expression3);
        if (0 > Number3)
          Number3 = 0;
        str38: String = str37 + Expression3 + Strings.Space(Number3);
        Expression4: String = "DEAD".to_owned();
        let mut Number4: i32 =  7 - Strings.Len(Expression4);
        if (0 > Number4)
          Number4 = 0;
        str39: String = str38 + Expression4 + Strings.Space(Number4);
        Expression5: String = "RETREAT".to_owned();
        let mut Number5: i32 =  9 - Strings.Len(Expression5);
        if (0 > Number5)
          Number5 = 0;
        str40: String = str39 + Expression5 + Strings.Space(Number5);
        Expression6: String = "ALIVE".to_owned();
        let mut Number6: i32 =  6 - Strings.Len(Expression6);
        if (0 > Number6)
          Number6 = 0;
        str41: String = str40 + Expression6 + Strings.Space(Number6) + "\r\n";
        let mut sfTypeCounter1: i32 =  this.game.Data.SFTypeCounter;
        for (let mut index120: i32 =  0; index120 <= sfTypeCounter1; index120 += 1)
        {
          let mut num164: i32 =  1;
          if (this.game.Data.SFTypeObj[index120].Ratio > 0)
            num164 = this.game.Data.SFTypeObj[index120].Ratio;
          if (numArray22[index120, 1] > 0 | numArray20[index120, 1] > 0 | numArray21[index120, 1] > 0)
          {
            Expression7: String = this.game.Data.SFTypeObj[index120].Name;
            if (Strings.Len(Expression7) > 29)
              Expression7 = Strings.Left(str41, 29);
            let mut Number7: i32 =  35 - Strings.Len(Expression7);
            if (0 > Number7)
              Number7 = 0;
            str42: String = str41 + Expression7 + Strings.Space(Number7);
            Expression8: String = Strings.Trim(Conversion.Str( (num164 * (numArray22[index120, 1] + numArray20[index120, 1] + numArray21[index120, 1]))));
            let mut Number8: i32 =  10 - Strings.Len(Expression8);
            if (0 > Number8)
              Number8 = 0;
            str43: String = str42 + Expression8 + Strings.Space(Number8);
            Expression9: String = Strings.Trim(Conversion.Str( (num164 * numArray22[index120, 1])));
            let mut Number9: i32 =  9 - Strings.Len(Expression9);
            if (0 > Number9)
              Number9 = 0;
            str44: String = str43 + Expression9 + Strings.Space(Number9);
            Expression10: String = Strings.Trim(Conversion.Str( (num164 * numArray20[index120, 1])));
            let mut Number10: i32 =  7 - Strings.Len(Expression10);
            if (0 > Number10)
              Number10 = 0;
            str45: String = str44 + Expression10 + Strings.Space(Number10);
            Expression11: String = Strings.Trim(Conversion.Str( (num164 * numArray21[index120, 1])));
            let mut Number11: i32 =  9 - Strings.Len(Expression11);
            if (0 > Number11)
              Number11 = 0;
            str46: String = str45 + Expression11 + Strings.Space(Number11);
            Expression12: String = Strings.Trim(Conversion.Str( (num164 * (numArray21[index120, 1] + numArray22[index120, 1]))));
            let mut Number12: i32 =  6 - Strings.Len(Expression12);
            if (0 > Number12)
              Number12 = 0;
            str41 = str46 + Expression12 + Strings.Space(Number12) + "\r\n";
          }
        }
        if (numArray31[0] > 0)
        {
          str47: String = str41 + "\r\n" + "DEFENDERS TOTALS\r\n";
          Expression13: String = "SUBFORMATIONTYPE".to_owned();
          let mut Number13: i32 =  35 - Strings.Len(Expression13);
          if (0 > Number13)
            Number13 = 0;
          str48: String = str47 + Expression13 + Strings.Space(Number13);
          Expression14: String = "INITIAL".to_owned();
          let mut Number14: i32 =  10 - Strings.Len(Expression14);
          if (0 > Number14)
            Number14 = 0;
          str49: String = str48 + Expression14 + Strings.Space(Number14);
          Expression15: String = "@FRONT";
          let mut Number15: i32 =  9 - Strings.Len(Expression15);
          if (0 > Number15)
            Number15 = 0;
          str50: String = str49 + Expression15 + Strings.Space(Number15);
          Expression16: String = "DEAD".to_owned();
          let mut Number16: i32 =  7 - Strings.Len(Expression16);
          if (0 > Number16)
            Number16 = 0;
          str51: String = str50 + Expression16 + Strings.Space(Number16);
          Expression17: String = "RETREAT".to_owned();
          let mut Number17: i32 =  9 - Strings.Len(Expression17);
          if (0 > Number17)
            Number17 = 0;
          str52: String = str51 + Expression17 + Strings.Space(Number17);
          Expression18: String = "ALIVE".to_owned();
          let mut Number18: i32 =  6 - Strings.Len(Expression18);
          if (0 > Number18)
            Number18 = 0;
          str41 = str52 + Expression18 + Strings.Space(Number18) + "\r\n";
        }
        let mut sfTypeCounter2: i32 =  this.game.Data.SFTypeCounter;
        for (let mut index121: i32 =  0; index121 <= sfTypeCounter2; index121 += 1)
        {
          let mut num165: i32 =  1;
          if (this.game.Data.SFTypeObj[index121].Ratio > 0)
            num165 = this.game.Data.SFTypeObj[index121].Ratio;
          if (numArray22[index121, 0] > 0 | numArray20[index121, 0] > 0 | numArray21[index121, 0] > 0)
          {
            Expression19: String = this.game.Data.SFTypeObj[index121].Name;
            if (Strings.Len(Expression19) > 29)
              Expression19 = Strings.Left(str41, 29);
            let mut Number19: i32 =  35 - Strings.Len(Expression19);
            if (0 > Number19)
              Number19 = 0;
            str53: String = str41 + Expression19 + Strings.Space(Number19);
            Expression20: String = Strings.Trim(Conversion.Str( (num165 * (numArray22[index121, 0] + numArray20[index121, 0] + numArray21[index121, 0]))));
            let mut Number20: i32 =  10 - Strings.Len(Expression20);
            if (0 > Number20)
              Number20 = 0;
            str54: String = str53 + Expression20 + Strings.Space(Number20);
            Expression21: String = Strings.Trim(Conversion.Str( (num165 * numArray22[index121, 0])));
            let mut Number21: i32 =  9 - Strings.Len(Expression21);
            if (0 > Number21)
              Number21 = 0;
            str55: String = str54 + Expression21 + Strings.Space(Number21);
            Expression22: String = Strings.Trim(Conversion.Str( (num165 * numArray20[index121, 0])));
            let mut Number22: i32 =  7 - Strings.Len(Expression22);
            if (0 > Number22)
              Number22 = 0;
            str56: String = str55 + Expression22 + Strings.Space(Number22);
            Expression23: String = Strings.Trim(Conversion.Str( (num165 * numArray21[index121, 0])));
            let mut Number23: i32 =  9 - Strings.Len(Expression23);
            if (0 > Number23)
              Number23 = 0;
            str57: String = str56 + Expression23 + Strings.Space(Number23);
            Expression24: String = Strings.Trim(Conversion.Str( (num165 * (numArray21[index121, 0] + numArray22[index121, 0]))));
            let mut Number24: i32 =  6 - Strings.Len(Expression24);
            if (0 > Number24)
              Number24 = 0;
            str41 = str57 + Expression24 + Strings.Space(Number24) + "\r\n";
          }
        }
        str58: String = str41 + "\r\n";
        Expression25: String = "ATTACKER".to_owned();
        let mut Number25: i32 =  10 - Strings.Len(Expression25);
        if (0 > Number25)
          Number25 = 0;
        str59: String = str58 + Strings.Space(20) + Expression25 + Strings.Space(Number25);
        Expression26: String = "DEFENDER".to_owned();
        let mut num166: i32 =  10 - Strings.Len(Expression26);
        if (0 > num166)
          num166 = 0;
        str60: String = str59 + Strings.Space(20) + Expression26 + Strings.Space(num166 + 50) + "\r\n";
        Expression27: String = "STAT".to_owned();
        let mut Number26: i32 =  20 - Strings.Len(Expression27);
        if (0 > Number26)
          Number26 = 0;
        str61: String = str60 + Expression27 + Strings.Space(Number26);
        Expression28: String = "INITIAL".to_owned();
        let mut Number27: i32 =  10 - Strings.Len(Expression28);
        if (0 > Number27)
          Number27 = 0;
        str62: String = str61 + Expression28 + Strings.Space(Number27);
        Expression29: String = "CURRENT".to_owned();
        let mut Number28: i32 =  20 - Strings.Len(Expression29);
        if (0 > Number28)
          Number28 = 0;
        str63: String = str62 + Expression29 + Strings.Space(Number28);
        Expression30: String = "INITIAL".to_owned();
        let mut Number29: i32 =  10 - Strings.Len(Expression30);
        if (0 > Number29)
          Number29 = 0;
        str64: String = str63 + Expression30 + Strings.Space(Number29);
        Expression31: String = "CURRENT".to_owned();
        let mut Number30: i32 =  10 - Strings.Len(Expression31);
        if (0 > Number30)
          Number30 = 0;
        str65: String = str64 + Expression31 + Strings.Space(Number30) + "\r\n";
        Expression32: String = "Readiness".to_owned();
        if (Strings.Len(Expression32) > 29)
          Expression32 = Strings.Left(str65, 29);
        let mut Number31: i32 =  20 - Strings.Len(Expression32);
        if (0 > Number31)
          Number31 = 0;
        str66: String = str65 + Expression32 + Strings.Space(Number31);
        Expression33: String = Strings.Trim(Conversion.Str( this.StartRdn[1]));
        let mut Number32: i32 =  10 - Strings.Len(Expression33);
        if (0 > Number32)
          Number32 = 0;
        str67: String = str66 + Expression33 + Strings.Space(Number32);
        Expression34: String = Strings.Trim(Conversion.Str( numArray23[1]));
        let mut Number33: i32 =  20 - Strings.Len(Expression34);
        if (0 > Number33)
          Number33 = 0;
        str68: String = str67 + Expression34 + Strings.Space(Number33);
        str69: String = Strings.Trim(Conversion.Str( this.StartRdn[0]));
        if (numArray31[0] < 1)
          str69 = "?";
        if (Operators.CompareString(str69, "-1", false) == 0)
          str69 = "?";
        let mut Number34: i32 =  10 - Strings.Len(str69);
        if (0 > Number34)
          Number34 = 0;
        str70: String = str68 + str69 + Strings.Space(Number34);
        str71: String = Strings.Trim(Conversion.Str( numArray23[0]));
        if (numArray31[0] < 1)
          str71 = "?";
        if (Operators.CompareString(str71, "-1", false) == 0)
          str71 = "?";
        let mut Number35: i32 =  10 - Strings.Len(str71);
        if (0 > Number35)
          Number35 = 0;
        str72: String = str70 + str71 + Strings.Space(Number35) + "\r\n";
        Expression35: String = "Experience".to_owned();
        if (Strings.Len(Expression35) > 29)
          Expression35 = Strings.Left(str72, 29);
        let mut Number36: i32 =  20 - Strings.Len(Expression35);
        if (0 > Number36)
          Number36 = 0;
        str73: String = str72 + Expression35 + Strings.Space(Number36);
        Expression36: String = Strings.Trim(Conversion.Str( this.StartXp[1]));
        let mut Number37: i32 =  10 - Strings.Len(Expression36);
        if (0 > Number37)
          Number37 = 0;
        str74: String = str73 + Expression36 + Strings.Space(Number37);
        Expression37: String = Strings.Trim(Conversion.Str( numArray24[1]));
        let mut Number38: i32 =  20 - Strings.Len(Expression37);
        if (0 > Number38)
          Number38 = 0;
        str75: String = str74 + Expression37 + Strings.Space(Number38);
        str76: String = Strings.Trim(Conversion.Str( this.StartXp[0]));
        if (numArray31[0] < 1)
          str76 = "?";
        if (Operators.CompareString(str76, "-1", false) == 0)
          str76 = "?";
        let mut Number39: i32 =  10 - Strings.Len(str76);
        if (0 > Number39)
          Number39 = 0;
        str77: String = str75 + str76 + Strings.Space(Number39);
        str78: String = Strings.Trim(Conversion.Str( numArray24[0]));
        if (numArray31[0] < 1)
          str78 = "?";
        if (Operators.CompareString(str78, "-1", false) == 0)
          str78 = "?";
        let mut Number40: i32 =  10 - Strings.Len(str78);
        if (0 > Number40)
          Number40 = 0;
        str79: String = str77 + str78 + Strings.Space(Number40) + "\r\n";
        Expression38: String = "Morale".to_owned();
        if (Strings.Len(Expression38) > 29)
          Expression38 = Strings.Left(str79, 29);
        let mut Number41: i32 =  20 - Strings.Len(Expression38);
        if (0 > Number41)
          Number41 = 0;
        str80: String = str79 + Expression38 + Strings.Space(Number41);
        Expression39: String = Strings.Trim(Conversion.Str( this.StartMor[1]));
        let mut Number42: i32 =  10 - Strings.Len(Expression39);
        if (0 > Number42)
          Number42 = 0;
        str81: String = str80 + Expression39 + Strings.Space(Number42);
        Expression40: String = Strings.Trim(Conversion.Str( numArray25[1]));
        let mut Number43: i32 =  20 - Strings.Len(Expression40);
        if (0 > Number43)
          Number43 = 0;
        str82: String = str81 + Expression40 + Strings.Space(Number43);
        str83: String = Strings.Trim(Conversion.Str( this.StartMor[0]));
        if (numArray31[0] < 1)
          str83 = "?";
        if (Operators.CompareString(str83, "-1", false) == 0)
          str83 = "?";
        let mut Number44: i32 =  10 - Strings.Len(str83);
        if (0 > Number44)
          Number44 = 0;
        str84: String = str82 + str83 + Strings.Space(Number44);
        str85: String = Strings.Trim(Conversion.Str( numArray25[0]));
        if (numArray31[0] < 1)
          str85 = "?";
        if (Operators.CompareString(str85, "-1", false) == 0)
          str85 = "?";
        let mut Number45: i32 =  10 - Strings.Len(str85);
        if (0 > Number45)
          Number45 = 0;
        str86: String = str84 + str85 + Strings.Space(Number45) + "\r\n";
        Expression41: String = "Entrenchment".to_owned();
        if (Strings.Len(Expression41) > 29)
          Expression41 = Strings.Left(str86, 29);
        let mut Number46: i32 =  20 - Strings.Len(Expression41);
        if (0 > Number46)
          Number46 = 0;
        str87: String = str86 + Expression41 + Strings.Space(Number46);
        Expression42: String = Strings.Trim(Conversion.Str( this.StartEntr[1]));
        let mut Number47: i32 =  10 - Strings.Len(Expression42);
        if (0 > Number47)
          Number47 = 0;
        str88: String = str87 + Expression42 + Strings.Space(Number47);
        Expression43: String = Strings.Trim(Conversion.Str( numArray26[1]));
        let mut Number48: i32 =  20 - Strings.Len(Expression43);
        if (0 > Number48)
          Number48 = 0;
        str89: String = str88 + Expression43 + Strings.Space(Number48);
        str90: String = Strings.Trim(Conversion.Str( this.StartEntr[0]));
        if (numArray31[0] < 1)
          str90 = "?";
        if (Operators.CompareString(str90, "-1", false) == 0)
          str90 = "?";
        let mut Number49: i32 =  10 - Strings.Len(str90);
        if (0 > Number49)
          Number49 = 0;
        str91: String = str89 + str90 + Strings.Space(Number49);
        str92: String = Strings.Trim(Conversion.Str( numArray26[0]));
        if (numArray31[0] < 1)
          str92 = "?";
        if (Operators.CompareString(str92, "-1", false) == 0)
          str92 = "?";
        let mut Number50: i32 =  10 - Strings.Len(str92);
        if (0 > Number50)
          Number50 = 0;
        str93: String = str91 + str92 + Strings.Space(Number50) + "\r\n" + "\r\n";
        if (this.game.TempCombat.BattleEnded > 0)
        {
          str94: String = this.game.EditObj.CombatOneSentence;
          if (this.game.EditObj.CombatOneSentenceCustom.Length > 0 & str94.Length > 0)
            str94 = str94 + "\r\n•" + this.game.EditObj.CombatOneSentenceCustom;
          else if (this.game.EditObj.CombatOneSentenceCustom.Length > 0)
            str94 = "•" + this.game.EditObj.CombatOneSentenceCustom;
          if (str94.Length > 1)
          {
            str95: String = str94.Replace(".", ".\r\n•");
            if (Operators.CompareString(Strings.Right(str95, 1), "•", false) == 0)
              str95 = Strings.Left(str95, Strings.Len(str95) - 2);
            str93 = str93 + str95 + "\r\n";
          }
        }
        let mut num167: i32 =  18;
        let mut num168: i32 =  rectangle1.X + num167;
        let mut num169: i32 =  rectangle1.Y + 12 + num167;
        let mut num170: i32 =  rectangle2.X - rectangle1.X + rectangle2.Width - num167 * 2 + 16;
        let mut num171: i32 =  rectangle1.Height - num167 * 2 - 10;
        tTexty: String;
        if (this.game.ScreenHeight <= 900)
          tTexty = "[element][type]text[/type][pos]50,50," + (num170 - 100).ToString() + "," + (num171 - 50).ToString() + ",1[/pos][fontname]LMmonoCaps-Regular.ttf[/fontname][fontsize]16[/fontsize][fontstyle]0[/fontstyle][lineheight]18[/lineheight][text]" + str93 + "[/text][color]0,0,0,255][/color][/element]";
        else
          tTexty = "[element][type]text[/type][pos]50,50," + (num170 - 100).ToString() + "," + (num171 - 50).ToString() + ",1[/pos][fontname]LMmonoCaps-Regular.ttf[/fontname][fontsize]20[/fontsize][fontstyle]0[/fontstyle][lineheight]26[/lineheight][text]" + str93 + "[/text][color]0,0,0,255][/color][/element]";
        tsubpart5 =  new UDSPartClass(this.game, num170, num171, tTexty, ref this.OwnBitmap, num168, num169, tAllGray: true);
        this.resolveId = this.AddSubPart(ref tsubpart5, num168, num169, num170, num171, 1);
      }
      graphics.Dispose();
      graphics = (Graphics) null;
      if (!Information.IsNothing( this.bufferBitmap))
      {
        this.bufferBitmap.Dispose();
        this.bufferBitmap = (Bitmap) null;
      }
      this.bufferBitmap = (Bitmap) this.OwnBitmap.Clone();
    }

    pub void DrawBlockies(
      qty1: i32,
      qty2: i32,
      col1: Color,
      col2: Color,
      Graphics g,
      x: i32,
      y: i32)
    {
      if (this.game.EditObj.CombatNumbers)
      {
        SizeF sizeF1 = SizeF::new();
        let mut num1: i32 =  0;
        num2: i32;
        if (qty1 > 0)
        {
          text: String = Strings.Trim(Conversion.Str( qty1));
          SizeF sizeF2 = g.MeasureString(text, this.game.MarcFont11);
          num3: i32;
          num2 = (int) Math.Round( ( num3 + sizeF2.Width));
        }
        if (qty2 > 0)
        {
          text: String = Strings.Trim(Conversion.Str( qty2));
          SizeF sizeF3 = g.MeasureString(text, this.game.MarcFont11);
          num2 = (int) Math.Round( ( num2 + sizeF3.Width));
        }
        num4: i32;
        if (num2 < 20)
          num4 = (int) Math.Round( (20 - num2) / 2.0);
        x += num4;
        SizeF sizeF4;
        if (qty1 > 0)
        {
          str: String = Strings.Trim(Conversion.Str( qty1));
          sizeF4 = g.MeasureString(str, this.game.MarcFont11);
          DrawMod.DrawTextColouredMarc(ref g, str, this.game.MarcFont11b, x - 1, y - 1, col1);
          DrawMod.DrawRectangle(ref g, x - 1, y - 1, (int) Math.Round( (sizeF4.Width + 1f)), (int) Math.Round( (sizeF4.Height + 0.0f)), (int) col1.R, (int) col1.G, (int) col1.B, (int) col1.A);
          num1 = (int) Math.Round( ( num1 + (2f + sizeF4.Width)));
        }
        if (qty2 <= 0)
          return;
        str1: String = Strings.Trim(Conversion.Str( qty2));
        sizeF4 = g.MeasureString(str1, this.game.MarcFont11);
        DrawMod.DrawTextColouredMarc(ref g, str1, this.game.MarcFont11b, x + num1 - 1, y - 1, col2);
        DrawMod.DrawRectangle(ref g, x + num1 - 1, y - 1, (int) Math.Round( (sizeF4.Width + 1f)), (int) Math.Round( (sizeF4.Height + 0.0f)), (int) col2.R, (int) col2.G, (int) col2.B, (int) col2.A);
      }
      else
      {
        int[] numArray1 = new int[100];
        int[] numArray2 = new int[100];
        let mut index1: i32 =  -1;
        if (qty1 >= 100)
          qty1 = 100;
        if (qty2 >= 100)
          qty2 = 100;
        while (qty1 > 0 | qty2 > 0)
        {
          if (qty1 >= 50)
          {
            index1 += 1;
            numArray1[index1] = 50;
            numArray2[index1] = 1;
            qty1 -= 50;
          }
          else if (qty2 >= 50)
          {
            index1 += 1;
            numArray1[index1] = 10;
            numArray2[index1] = 2;
            qty2 -= 10;
          }
          else if (qty1 >= 10)
          {
            index1 += 1;
            numArray1[index1] = 10;
            numArray2[index1] = 1;
            qty1 -= 10;
          }
          else if (qty2 >= 10)
          {
            index1 += 1;
            numArray1[index1] = 10;
            numArray2[index1] = 2;
            qty2 -= 10;
          }
          else if (qty1 >= 5 & qty2 >= 5)
          {
            index1 += 1;
            numArray1[index1] = 10;
            numArray2[index1] = 3;
            qty1 -= 5;
            qty2 -= 5;
          }
          else if (qty1 >= 1)
          {
            index1 += 1;
            numArray1[index1] = 1;
            numArray2[index1] = 1;
            --qty1;
          }
          else if (qty2 >= 1)
          {
            index1 += 1;
            numArray1[index1] = 1;
            numArray2[index1] = 2;
            --qty2;
          }
        }
        let mut num5: i32 =  0;
        let mut num6: i32 =  0;
        let mut num7: i32 =  0;
        let mut num8: i32 =  -1;
        let mut num9: i32 =  0;
        let mut num10: i32 =  0;
        do
        {
          let mut num11: i32 =  index1;
          for (let mut index2: i32 =  0; index2 <= num11; index2 += 1)
          {
            if (numArray1[index2] == 50 & num10 == 0 | numArray1[index2] == 10 & num10 == 1 | numArray1[index2] == 1 & num10 == 2)
            {
              if (numArray1[index2] == 50)
              {
                if (numArray2[index2] == 1)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 5, 4, (int) col1.R, (int) col1.G, (int) col1.B, (int) col1.A);
                if (numArray2[index2] == 2)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 5, 4, (int) col2.R, (int) col2.G, (int) col2.B, (int) col2.A);
                num5 += 6;
                num7 = num6 + 5;
              }
              if (numArray1[index2] == 10)
              {
                if (numArray2[index2] == 1)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 2, 4, (int) col1.R, (int) col1.G, (int) col1.B, (int) col1.A);
                if (numArray2[index2] == 2)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 2, 4, (int) col2.R, (int) col2.G, (int) col2.B, (int) col2.A);
                if (numArray2[index2] == 3)
                {
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 1, 4, (int) col1.R, (int) col1.G, (int) col1.B, (int) col1.A);
                  DrawMod.DrawBlock(ref g, x + num5 + 1, y + num6, 1, 4, (int) col2.R, (int) col2.G, (int) col2.B, (int) col2.A);
                }
                num5 += 3;
                num7 = num6 + 5;
              }
              if (numArray1[index2] == 1)
              {
                if (num6 == 0 & num9 == 0 & num5 > 0)
                {
                  num6 = num7;
                  num5 = 0;
                }
                num9 += 1;
                if (numArray2[index2] == 1)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 2, 2, (int) col1.R, (int) col1.G, (int) col1.B, (int) col1.A);
                if (numArray2[index2] == 2)
                  DrawMod.DrawBlock(ref g, x + num5, y + num6, 2, 2, (int) col2.R, (int) col2.G, (int) col2.B, (int) col2.A);
                if (num8 == -1)
                  num8 = num5;
                num5 += 3;
                num7 = num6 + 3;
              }
              if (num5 > 15)
              {
                num5 = 0;
                num6 = num7;
                if (num8 != -1)
                  num5 = num8;
              }
            }
          }
          num10 += 1;
        }
        while (num10 <= 2);
      }
    }

    pub fn GetSfNrBitmap(sfnr: i32, att: i32) -> i32
    {
      let mut type: i32 =  this.game.Data.SFObj[sfnr].Type;
      let mut symbolSprite2Id: i32 =  this.game.Data.SFTypeObj[type].SymbolSprite2ID;
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
              symbolSprite2Id = this.game.Data.SFTypeObj[type].ExtraSymbolSprite2ID[index2];
          }
        }
        else if (this.game.Data.PeopleObj[this.game.Data.SFObj[sfnr].People].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[type].ExtraCounter;
          for (let mut index3: i32 =  0; index3 <= extraCounter; index3 += 1)
          {
            if (this.game.Data.SFTypeObj[type].ExtraCode[index3] == this.game.Data.PeopleObj[this.game.Data.SFObj[sfnr].People].ExtraGraphicUse)
              symbolSprite2Id = this.game.Data.SFTypeObj[type].ExtraSymbolSprite2ID[index3];
          }
        }
      }
      return symbolSprite2Id;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          if (nr > 0 & this.okid > 0)
          {
            this.ForwardKey = false;
            windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.okid)] + 1, this.SubPartY[this.SubpartNr(this.okid)] + 1, 1);
            windowReturnClass.SetFlag(true);
          }
          if (this.game.TempCombat.BattleEnded > 0)
          {
            if (!this.ForwardKey)
            {
              this.game.EditObj.TempCoordList.DeActivate();
              this.game.EditObj.CombatOneSentence = "";
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.TargetX = -1;
              this.game.EditObj.TargetY = -1;
              this.game.EditObj.OrderBombMode = 0;
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          else
          {
            if (nr == 32 & this.roundId > 0)
            {
              this.ForwardKey = false;
              windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.roundId)] + 1, this.SubPartY[this.SubpartNr(this.roundId)] + 1, 1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (nr == 27 & this.playId > 0)
            {
              this.ForwardKey = false;
              windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.playId)] + 1, this.SubPartY[this.SubpartNr(this.playId)] + 1, 1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter1: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter1; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut maxAttPage: i32 =  this.maxAttPage;
            for (let mut index2: i32 =  0; index2 <= maxAttPage; index2 += 1)
            {
              if (this.tabid[1, index2] == this.SubPartID[index1])
              {
                this.attPage = index2;
                this.SubPartFlag[index1] = true;
                this.dostuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            let mut maxDefPage: i32 =  this.maxDefPage;
            for (let mut index3: i32 =  0; index3 <= maxDefPage; index3 += 1)
            {
              if (this.tabid[0, index3] == this.SubPartID[index1])
              {
                this.defPage = index3;
                this.SubPartFlag[index1] = true;
                this.dostuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            if (this.SubPartID[index1] == this.tabup[0] & this.defPage < this.maxDefPage)
            {
              this += 1.defPage;
              this.SubPartFlag[index1] = true;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (this.SubPartID[index1] == this.tabup[1] & this.attPage < this.maxAttPage)
            {
              this += 1.attPage;
              this.SubPartFlag[index1] = true;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (this.SubPartID[index1] == this.tabdown[0] & this.defPage > 0)
            {
              --this.defPage;
              this.SubPartFlag[index1] = true;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (this.SubPartID[index1] == this.tabdown[1] & this.attPage > 0)
            {
              --this.attPage;
              this.SubPartFlag[index1] = true;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
          }
        }
        let mut subPartCounter2: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter2; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 =  this.SubPartID[index];
            if (num == this.typeid)
            {
              if (this.showdetail <= 0)
              {
                this.showdetail = 1;
                this.game.EditObj.CombatTextual = true;
              }
              else
              {
                this.showdetail = 0;
                this.game.EditObj.CombatTextual = false;
              }
              this.game.EditObj.PopupValue = 7;
              this.animList = SimpleList::new();
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.detailid)
            {
              this.game.EditObj.PopupValue = 8;
              this.bufferBitmap.Dispose();
              this.bufferBitmap = (Bitmap) null;
              windowReturnClass1.AddCommand(5, 14);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.playId)
            {
              this.playBattle = true;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.roundId)
            {
              this.game.TempCombat.DoRound();
              this.dostuff();
              this.lasttime = DateAndTime.Now;
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.zoom0id)
            {
              this.useZoom = 0;
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              this.crmSet = false;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.zoom1id)
            {
              this.useZoom = 1;
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              this.crmSet = false;
              this.dostuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.resolveId)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num == this.okid)
            {
              Expression: WindowReturnClass = this.EndTheBattle();
              this.game.EditObj.TempCoordList.DeActivate();
              this.game.EditObj.CombatOneSentence = "";
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.TargetX = -1;
              this.game.EditObj.TargetY = -1;
              SoundMod.STopEventWave();
              this.bufferBitmap.Dispose();
              this.bufferBitmap = (Bitmap) null;
              windowReturnClass1.AddCommand(6, 0);
              if (!Information.IsNothing( Expression))
                windowReturnClass1 = Expression;
              this.game.EditObj.se1_SelectAssetButton = -1;
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub EndTheBattle: WindowReturnClass()
    {
      this.game.TempCombat.EndBattle();
      if (this.game.EditObj.DoCardSlot > -1)
      {
        windowReturnClass: WindowReturnClass;
        if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot > -1)
        {
          this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
          this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot;
          this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaValue;
          this.game.EditObj.PopupValue = 1;
          this.bufferBitmap.Dispose();
          this.bufferBitmap = (Bitmap) null;
          windowReturnClass.AddCommand(5, 10);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].UnitSelect)
        {
          this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
          this.game.EditObj.PopupValue = 3;
          this.bufferBitmap.Dispose();
          this.bufferBitmap = (Bitmap) null;
          windowReturnClass.AddCommand(5, 10);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
        this.game.ProcessingObj.PlayCard(this.game.EditObj.DoCardSlot);
        if (Strings.Len(this.game.Data.LoadGame) > 0)
        {
          this.game.FormRef.Cursor = Cursors.WaitCursor;
          Form formRef =  this.game.FormRef;
          this.game.HandyFunctionsObj.LoadGameNow();
          this.game.FormRef = (Form1) formRef;
          this.game.FormRef.Cursor = Cursors.Default;
          this.bufferBitmap.Dispose();
          this.bufferBitmap = (Bitmap) null;
          windowReturnClass.AddCommand(3, 4);
          return windowReturnClass;
        }
        let mut Number: i32 =  0;
        let mut locCounter: i32 =  this.game.Data.LocCounter;
        for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
          {
            let mut index: i32 =  0;
            do
            {
              if (this.game.Data.LocObj[locnr].Production[index] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index]).result)
              {
                Number += 1;
                this.game.Data.LocObj[locnr].Production[index] = -1;
                this.game.Data.LocObj[locnr].ProdPointRemainder[index] = 0;
                this.game.Data.LocObj[locnr].ProdPercent[index] = 0;
              }
              index += 1;
            }
            while (index <= 3);
          }
        }
        if (Number > 0)
        {
          let mut num: i32 =  (int) Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
        }
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
        {
          this.game.EditObj.PopupValue = 0;
          this.bufferBitmap.Dispose();
          this.bufferBitmap = (Bitmap) null;
          this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
          windowReturnClass.AddCommand(5, 10);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      windowReturnClass1: WindowReturnClass;
      return windowReturnClass1;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!this.game.EditObj.CombatSim)
      {
        if (this.game.TempCombat.BattleEnded == 0 & (this.game.EditObj.AutoCombat | this.playBattle))
        {
          TimeSpan timeSpan = DateAndTime.Now.Subtract(this.lasttime);
          if (timeSpan.Milliseconds + timeSpan.Seconds * 1000 > 1000 | this.game.TempCombat.CombatRound == 0)
          {
            this.game.TempCombat.DoRound();
            this.animList = SimpleList::new();
            this.dostuff();
            this.lasttime = DateAndTime.Now;
            windowReturnClass.SetFlag(true);
            if (this.game.TempCombat.BattleEnded <= 0)
              ;
            return windowReturnClass;
          }
        }
        TimeSpan timeSpan1 = DateAndTime.Now.Subtract(this.lastAnimTime);
        if (timeSpan1.Milliseconds + timeSpan1.Seconds * 1000 > 100 & this.animList.Counter > -1)
        {
          for (let mut counter: i32 =  this.animList.Counter; counter >= 0; counter += -1)
          {
            if (this.animList.Weight[counter] != -1)
            {
              if (this.animList.Weight[counter] <= 31)
              {
                if (this.animList.Data5[counter] == 1)
                {
                  int[] weight = this.animList.Weight;
                  int[] numArray = weight;
                  let mut index1: i32 =  counter;
                  let mut index2: i32 =  index1;
                  let mut num: i32 =  weight[index1] + 4;
                  numArray[index2] = num;
                }
                else
                {
                  int[] weight = this.animList.Weight;
                  int[] numArray = weight;
                  let mut index3: i32 =  counter;
                  let mut index4: i32 =  index3;
                  let mut num: i32 =  weight[index3] + 3;
                  numArray[index4] = num;
                }
              }
              else if (this.animList.Weight[counter] >= 32)
              {
                if (this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.animList.Id[counter])].IKilled == this.game.TempCombat.CombatRound)
                  this.animList.Weight[counter] = -1;
                else
                  this.animList.Remove(counter);
              }
            }
          }
          if (this.animList.Counter > -1 & !Information.IsNothing( this.bufferBitmap))
          {
            Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
            ref Graphics local1 = ref graphics;
            ref local2: Bitmap = ref this.bufferBitmap;
            ref local3: Bitmap = ref this.OwnBitmap;
            Rectangle rectangle1 = Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height - 55);
            let mut srcrect1: &Rectangle = &rectangle1
            Rectangle rectangle2 = Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height - 55);
            let mut destrect1: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2Fast(ref local1, ref local2, ref local3, srcrect1, destrect1);
            for (let mut counter: i32 =  this.animList.Counter; counter >= 0; counter += -1)
            {
              let mut num1: i32 =  this.animList.Weight[counter];
              if (num1 > 0)
              {
                if (this.animList.Data5[counter] == 1)
                {
                  ref Graphics local4 = ref graphics;
                  bitmap: Bitmap = BitmapStore.GetBitmap(this.game.EXPLOSION, 1);
                  ref local5: Bitmap = ref bitmap;
                  rectangle2 = Rectangle::new(128 * (num1 - 1), 0, 128, 96);
                  let mut srcrect2: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(this.animList.Data1[counter] - (int) Math.Round( this.animList.Data3[counter] * 0.5), this.animList.Data2[counter] - (int) Math.Round( this.animList.Data4[counter] * 0.5), this.animList.Data3[counter] * 2, this.animList.Data4[counter] * 2);
                  let mut destrect2: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local4, ref local5, srcrect2, destrect2);
                }
                else
                {
                  float num2 = 1f;
                  if (num1 < 23)
                    num2 = 1f - Math.Min(1f,  (23 - num1) / 6f);
                  ref Graphics local6 = ref graphics;
                  bitmap: Bitmap = BitmapStore.GetBitmap(this.game.EXPLOSION, 1);
                  ref local7: Bitmap = ref bitmap;
                  rectangle2 = Rectangle::new(128 * (num1 - 1), 0, 128, 96);
                  let mut srcrect3: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(this.animList.Data1[counter] - (int) Math.Round( this.animList.Data3[counter] * 0.5), this.animList.Data2[counter] - (int) Math.Round( this.animList.Data4[counter] * 0.5), this.animList.Data3[counter] * 2, this.animList.Data4[counter] * 2);
                  let mut destrect3: &Rectangle = &rectangle1
                  double a =  num2;
                  DrawMod.DrawSimplePart2ColouredNew(ref local6, ref local7, srcrect3, destrect3, 1f, 1f, 1f,  a);
                }
              }
            }
            graphics.Dispose();
          }
          this.lastAnimTime = DateAndTime.Now;
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn PopUpRefresh()
    {
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaMap = -1;
      this.game.EditObj.AreaValue = -1;
      this.dostuff();
    }
  }
}
