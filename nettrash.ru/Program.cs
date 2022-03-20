using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;

namespace nettrash.ru
{
	public class Program
	{
		#region Public methods



		public static void Main(string[] args)
		{
			CreateHostBuilder(args).Build().Run();
		}

		public static IHostBuilder CreateHostBuilder(string[] args) =>
			Host.CreateDefaultBuilder(args)
				.ConfigureWebHostDefaults(webBuilder =>
				{
					webBuilder.UseStartup<Startup>();
				});



		#endregion
	}
}
